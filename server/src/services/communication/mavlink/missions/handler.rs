
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::{broadcast::Receiver, Mutex}, time};
use mavlink::{MavHeader, common::*};

use crate::models::missions::*;
use crate::models::events::ClientEvent;
use super::{super::context::MavlinkContext, protocol};

const MISSION_RESEND_INTERVAL: time::Duration = time::Duration::from_millis(2000);

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
        let vehicle_mission = context.registry.missions.vehicle_mission(&mission_id).await;
        if let Err(err) = vehicle_mission {
            log::error!("Error obtaining vehicle mission: {}", err);
            return None;
        }

        match context.mav_id_from_vehicle_id(&vehicle_mission.unwrap().vehicle_id) {
            Some(mav_id) => Some(mav_id),
            None => {
                log::error!("No MAVLink vehicle for mission: {:?}", &mission_id);
                None
            }
        }
    }

    async fn download_mission(&mut self, mission_id: MissionId) {
        let mav_id = match self.mav_id_for_mission_id(&mission_id).await {
            Some(mav_id) => mav_id,
            None => return
        };
        let context = self.context.lock().await;

        let status = context.registry.missions.mission_status(&mission_id).await;
        if let Err(err) = status {
            log::error!("Error getting mission status: {}", err);
            return;
        }
        let mut status = status.unwrap();
        match status.state {
            MissionUpdateState::NotActual {} |
            MissionUpdateState::Actual { .. } => {}
            _ => {
                log::info!("Mission update operation is in progress, skipping download");
                return;
            }
        }

        log::info!("Starting mission download for MAVLink {}", mav_id);
        status.state = MissionUpdateState::PrepareDownload {};
        self.mav_active_statuses.insert(mav_id, status.clone());
        if let Err(err) = context.registry.missions.update_status(&status).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    async fn cancel_mission_state(&mut self, mission_id: MissionId) {
        let context = self.context.lock().await;

        let status = context.registry.missions.mission_status(&mission_id).await;
        if let Err(err) = status {
            log::error!("Error getting mission status: {}", err);
            return;
        }
        let mut status = status.unwrap();
        status.state = MissionUpdateState::NotActual {};
        if let Err(err) = context.registry.missions.update_status(&status).await {
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
                log::info!("TODO: Upload mission: {:?}", &mission_id);
            }
            ClientEvent::ClearMission { mission_id } => {
                log::info!("TODO: Clear mission: {:?}", &mission_id);
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
            MissionUpdateState::Upload { total, progress } => todo!(),
            MissionUpdateState::Clearing {} => todo!(),
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
            match context.registry.missions.mission_route(&status.id).await {
                Ok(mut route) => {
                    if route.items.len() > data.count as usize {
                        route.items.truncate(data.count as usize);
                        if let Err(err) = context.registry.missions.update_route(&route).await {
                            log::error!("Error updating mission route: {}", err);
                        }
                    }
                },
                Err(err) => {
                    log::error!("Error getting mission route: {}", err);
                }
            }

            if let Err(err) = context.registry.missions.update_status(&status).await {
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
                log::info!("Unexpected mission item sequence: {} (expected: {})", data.seq, progress);
                return;
            }
            log::info!("Got item {} from MAVLink {}", data.seq, mav_id);

            let context = self.context.lock().await;

            // Add route item, zero-based index
            if let Err(err) = context.registry.missions.set_route_item(
                &status.id,
                protocol::mission_route_item_from_mavlink(data),
                progress - 1
            ).await {
                log::error!("Error setting mission route item: {}", err);
            }

            if progress >= total {
                log::info!("Mission download completed for MAVLink {}", mav_id);
                status.state = MissionUpdateState::Actual { total };
            } else {
                status.state = MissionUpdateState::Download { total, progress: progress + 1 };
            }

            if let Err(err) = context.registry.missions.update_status(&status).await {
                log::error!("Error updating mission status: {}", err);
            }
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
            _ => {}
        }
    }
}