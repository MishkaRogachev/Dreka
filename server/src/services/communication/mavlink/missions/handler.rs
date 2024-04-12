
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::{broadcast::Receiver, Mutex}, time};
use mavlink::{MavHeader, common::*};

use crate::models::missions::*;
use crate::models::events::ClientEvent;
use super::{super::context::MavlinkContext, protocol};

const MISSION_RESEND_INTERVAL: time::Duration = time::Duration::from_millis(2000);

// TODO: universal Handler, get rid of mavlink context 
pub struct MissionHandler {
    context: Arc<Mutex<MavlinkContext>>,
    client_events_rx: Receiver<ClientEvent>,
    mav_active_statuses: HashMap<u8, MissionStatus>,
    mission_statuses_last_sent: HashMap<MissionId, time::Instant>,
}

impl MissionHandler {
    pub fn new(context: Arc<Mutex<MavlinkContext>>, client_events_rx: Receiver<ClientEvent>) -> Self {
        Self {
            context,
            client_events_rx,
            mav_active_statuses: HashMap::new(),
            mission_statuses_last_sent: HashMap::new()
        }
    }

    async fn mav_id_for_mission_id(&self, mission_id: &MissionId) -> Option<u8> {
        let context = self.context.lock().await;
        let assignment = context.dal.mission_assignment(&mission_id).await;
        if let Err(err) = assignment {
            log::error!("Error obtaining mission assignment: {}", err);
            return None;
        }

        match context.mav_id_from_vehicle_id(&assignment.unwrap().vehicle_id) {
            Some(mav_id) => Some(mav_id),
            None => {
                log::error!("No MAVLink vehicle for mission: {:?}", &mission_id);
                None
            }
        }
    }

    async fn activate_status(&mut self, mission_id: &MissionId) -> Option<MissionStatus> {
        let context = self.context.lock().await;
        let status = context.dal.mission_status(mission_id).await;
        if let Err(err) = status {
            log::error!("Error getting mission status: {}", err);
            return None;
        }
        let status = status.unwrap();
        match status.state {
            MissionUpdateState::NotActual {} |
            MissionUpdateState::Actual { .. } => Some(status),
            _ => {
                log::info!("Another mission operation is in progress, skipping download");
                return None;
            }
        }
    }

    async fn download_mission(&mut self, mission_id: MissionId) {
        let mav_id = match self.mav_id_for_mission_id(&mission_id).await {
            Some(mav_id) => mav_id,
            None => return
        };

        let mut status = match self.activate_status(&mission_id).await {
            Some(status) => status,
            None => return
        };

        log::info!("Download mission for MAVLink {}", mav_id);
        status.state = MissionUpdateState::PrepareDownload {};
        self.mav_active_statuses.insert(mav_id, status.clone());

        let context = self.context.lock().await;
        if let Err(err) = context.dal.update_mission_status(status).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    async fn upload_mission(&mut self, mission_id: MissionId) {
        let mav_id = match self.mav_id_for_mission_id(&mission_id).await {
            Some(mav_id) => mav_id,
            None => return
        };

        let mut status = match self.activate_status(&mission_id).await {
            Some(status) => status,
            None => return
        };

        let context = self.context.lock().await;
        let route = context.dal.mission_route(&mission_id).await;
        if let Err(err) = route {
            log::error!("Error getting mission route: {}", err);
            return;
        }
        let total = route.unwrap().items.len() as u16;
        if total == 0 {
            log::info!("Empty mission, skipping upload");
            return;
        }

        log::info!("Upload mission ({} items) for MAVLink {}", total, mav_id);
        status.state = MissionUpdateState::PrepareUpload { total };
        self.mav_active_statuses.insert(mav_id, status.clone());

        if let Err(err) = context.dal.update_mission_status(status).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    async fn clear_mission(&mut self, mission_id: MissionId) {
        let mav_id = match self.mav_id_for_mission_id(&mission_id).await {
            Some(mav_id) => mav_id,
            None => return
        };

        let mut status = match self.activate_status(&mission_id).await {
            Some(status) => status,
            None => return
        };

        status.state = MissionUpdateState::Clearing {};
        self.mav_active_statuses.insert(mav_id, status.clone());

        let context = self.context.lock().await;
        if let Err(err) = context.dal.update_mission_status(status).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    async fn cancel_mission_state(&mut self, mission_id: MissionId) {
        let context = self.context.lock().await;

        let status = context.dal.mission_status(&mission_id).await;
        if let Err(err) = status {
            log::error!("Error getting mission status: {}", err);
            return;
        }

        log::info!("Cancel mission operation");
        let mut status = status.unwrap();
        status.state = MissionUpdateState::NotActual {};
        if let Err(err) = context.dal.update_mission_status(status).await {
            log::error!("Error updating mission status: {}", err);
        }

        self.mav_active_statuses.retain(|_, status| status.id != mission_id);
        self.mission_statuses_last_sent.remove(&mission_id);
    }

    async fn handle_client_event(&mut self, event: ClientEvent) {
        match event {
            ClientEvent::DownloadMission { mission_id } => {
                self.download_mission(mission_id).await;
            }
            ClientEvent::UploadMission { mission_id } => {
                self.upload_mission(mission_id).await;
            }
            ClientEvent::ClearMission { mission_id } => {
                self.clear_mission(mission_id).await;
            }
            ClientEvent::CancelMissionState { mission_id } => {
                self.cancel_mission_state(mission_id).await;
            }
            _ => {}
        }
    }

    async fn process_status_to_message(&self, mav_id: &u8, status: &MissionStatus) -> Option<MavMessage> {
        match status.state {
            MissionUpdateState::PrepareDownload {} => {
                return Some(protocol::mission_request_list(mav_id));
            },
            MissionUpdateState::Download { total: _, progress } => {
                return Some(protocol::request_mission_item(mav_id, progress));
            },
            MissionUpdateState::PrepareUpload { total } => {
                // NOTE: +1 for HOME item
                return Some(protocol::send_mission_count(mav_id, total + 1));
            },
            MissionUpdateState::Upload { total: _, progress } => {
                let context = self.context.lock().await;

                if progress == 0 {
                    log::warn!("Requested home item through upload sequence, skipping");
                    return None;
                }

                let route = context.dal.mission_route(&status.id).await;
                if let Err(err) = route {
                    log::error!("Error getting mission route: {}", err);
                    return None;
                }
                let route = route.unwrap();
                let item = route.items.get((progress - 1) as usize); // NOTE: -1 for HOME item
                if item.is_none() {
                    log::error!("No mission item at index {}", progress);
                    return None;
                }
                return protocol::send_mission_item(mav_id, item.unwrap(), progress);
            },
            MissionUpdateState::Clearing {} => {
                return Some(protocol::send_mission_clear(mav_id));
            },
            _ => None
        }
    }

    async fn process_count(&mut self, mav_id: u8, data: &MISSION_COUNT_DATA) {
        let status = match self.mav_active_statuses.get_mut(&mav_id) {
            Some(mav_id) => mav_id,
            None => return
        };

        if let MissionUpdateState::PrepareDownload {} = status.state {
            log::info!("Got total count {} from MAVLink {}", data.count, mav_id);

            let context = self.context.lock().await;

            // Update status
            status.state = if data.count == 0 {
                MissionUpdateState::Actual { total: 0 }
            } else {
                MissionUpdateState::Download { total: data.count, progress: 1 }
            };

            // Crop mission items
            match context.dal.mission_route(&status.id).await {
                Ok(mut route) => {
                    if route.items.len() > data.count as usize {
                        route.items.truncate(data.count as usize);
                        if let Err(err) = context.dal.update_route(route).await {
                            log::error!("Error updating mission route: {}", err);
                        }
                    }
                },
                Err(err) => {
                    log::error!("Error getting mission route: {}", err);
                }
            }

            if let Err(err) = context.dal.update_mission_status(status.clone()).await {
                log::error!("Error updating mission status: {}", err);
            }
        }
    }

    async fn process_item_int(&mut self, mav_id: u8, data: &MISSION_ITEM_INT_DATA) {
        let status = match self.mav_active_statuses.get_mut(&mav_id) {
            Some(mav_id) => mav_id,
            None => return
        };

        if let MissionUpdateState::Download { total, progress } = status.state {
            if data.seq != progress {
                log::warn!("Unexpected mission item {} from MAVLink {}", data.seq, mav_id);
                return;
            }
            log::info!("Got item {} from MAVLink {}", data.seq, mav_id);

            if data.seq == 0 {
                log::warn!("Requested home item through download sequence, skipping");
                // TODO: send home item
                return;
            }

            let context = self.context.lock().await;

            // Add route item, zero-based index
            if let Err(err) = context.dal.upsert_route_item(
                &status.id,
                protocol::mission_route_item_from_mavlink(data),
                progress - 1 // NOTE: -1 for HOME item
            ).await {
                log::error!("Error setting mission route item: {}", err);
            }

            if progress >= total {
                log::info!("Mission download completed for MAVLink {}", mav_id);
                // TODO: send ACK
                status.state = MissionUpdateState::Actual { total };
            } else {
                status.state = MissionUpdateState::Download { total, progress: progress + 1 };
            }

            if let Err(err) = context.dal.update_mission_status(status.clone()).await {
                log::error!("Error updating mission status: {}", err);
            }
        }
    }

    async fn process_item_request(&mut self, mav_id: u8, data: &MISSION_REQUEST_DATA) {
        let status = match self.mav_active_statuses.get_mut(&mav_id) {
            Some(mav_id) => mav_id,
            None => return
        };

        let total = match status.state {
            MissionUpdateState::PrepareUpload { total } => total,
            MissionUpdateState::Upload { total, progress: _ } => total,
            _ => {
                log::info!("Unexpected mission item {} requested from MAVLink {}", data.seq, mav_id);
                return;
            }
        };

        log::info!("Mission item {} requested from MAVLink {}", data.seq, mav_id);

        let context = self.context.lock().await;
        status.state = MissionUpdateState::Upload { total, progress: data.seq };
        if let Err(err) = context.dal.update_mission_status(status.clone()).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    async fn process_ack(&mut self, mav_id: u8, data: &MISSION_ACK_DATA) {
        let status = match self.mav_active_statuses.get_mut(&mav_id) {
            Some(mav_id) => mav_id,
            None => return
        };

        match data.mavtype {
            MavMissionResult::MAV_MISSION_ACCEPTED => {
                log::info!("Mission operation accepted by MAVLink {}", mav_id);
                match status.state {
                    MissionUpdateState::Upload { total, progress: _ } => {
                        status.state = MissionUpdateState::Actual { total };
                    },
                    MissionUpdateState::Clearing {} => {
                        status.state = MissionUpdateState::Actual { total: 0 };
                    },
                    _ => {}
                }
            },
            MavMissionResult::MAV_MISSION_OPERATION_CANCELLED => {
                log::info!("Mission operation canceled for MAVLink {}", mav_id);
                status.state = MissionUpdateState::NotActual {};
            },
            _ => {
                log::warn!("Mission operation error for MAVLink {}", mav_id);
                status.state = MissionUpdateState::NotActual {};
            },
        }

        let context = self.context.lock().await;
        if let Err(err) = context.dal.update_mission_status(status.clone()).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    async fn collect_statuses_messages(&mut self) -> Vec<MavMessage> {
        let mut messages = Vec::new();

        // Collect messages for active statuses
        for (mav_id, status) in self.mav_active_statuses.iter() {
            let now = time::Instant::now();
            let last_sent = self.mission_statuses_last_sent.get(&status.id);
            if last_sent.is_none() || now.duration_since(*last_sent.unwrap()) >= MISSION_RESEND_INTERVAL {
                if let Some(message) = self.process_status_to_message(mav_id, status).await {
                    messages.push(message);
                    self.mission_statuses_last_sent.insert(status.id.clone(), now);
                }
            }
        }

        // Remove unactive statuses
        self.mav_active_statuses.retain(|_, status| {
            match status.state {
                MissionUpdateState::NotActual {} |
                MissionUpdateState::Actual { .. } => false,
                _ => true,
            }
        });

        return messages;
    }

    pub async fn prepare_messages(&mut self) -> Vec<MavMessage> {
        match self.client_events_rx.try_recv() {
            Ok(event) => self.handle_client_event(event).await,
            Err(err) => {
                if err != tokio::sync::broadcast::error::TryRecvError::Empty {
                    log::error!("RX error: {}", err);
                }
            }
        }
        self.collect_statuses_messages().await
    }

    pub async fn handle_message(&mut self, header: &MavHeader, msg: &MavMessage) {
        match msg {
            MavMessage::MISSION_COUNT(data) => {
                self.process_count(header.system_id, data).await;
            },
            MavMessage::MISSION_ITEM_INT(data) => {
                self.process_item_int(header.system_id, data).await;
            },
            MavMessage::MISSION_REQUEST(data) => {
                self.process_item_request(header.system_id, data).await;
            }
            MavMessage::MISSION_ACK(ack_data) => {
                return self.process_ack(header.system_id, ack_data).await;
            },
            _ => {}
        }
    }
}
