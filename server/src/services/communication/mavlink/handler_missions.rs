
use mavlink::common::*;

use crate::models::missions::*;
use super::{handler, protocol::missions as protocol};

const MISSION_RESEND_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_millis(2000);

impl handler::Handler {
    async fn activate_status(&mut self, mission_id: &MissionId) -> Option<MissionStatus> {
        let status = self.dal.mission_status(mission_id).await;
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

    pub async fn download_mission(&mut self, mission_id: MissionId) {
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
        self.mav_mission_operation_statuses.insert(mav_id, status.clone());

        if let Err(err) = self.dal.update_mission_status(status).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    pub async fn upload_mission(&mut self, mission_id: MissionId) {
        let mav_id = match self.mav_id_for_mission_id(&mission_id).await {
            Some(mav_id) => mav_id,
            None => return
        };

        let mut status = match self.activate_status(&mission_id).await {
            Some(status) => status,
            None => return
        };

        let route = self.dal.mission_route(&mission_id).await;
        if let Err(err) = route {
            log::error!("Error getting mission route: {}", err);
            return;
        }
        // NOTE: +1 for HOME item
        let total = (route.unwrap().items.len() + 1) as u16;

        log::info!("Upload mission ({} items) for MAVLink {}", total, mav_id);
        status.state = MissionUpdateState::PrepareUpload { total };
        self.mav_mission_operation_statuses.insert(mav_id, status.clone());

        if let Err(err) = self.dal.update_mission_status(status).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    pub async fn clear_mission(&mut self, mission_id: MissionId) {
        let mav_id = match self.mav_id_for_mission_id(&mission_id).await {
            Some(mav_id) => mav_id,
            None => return
        };

        let mut status = match self.activate_status(&mission_id).await {
            Some(status) => status,
            None => return
        };

        status.state = MissionUpdateState::Clearing {};
        self.mav_mission_operation_statuses.insert(mav_id, status.clone());

        if let Err(err) = self.dal.update_mission_status(status).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    pub async fn cancel_mission_state(&mut self, mission_id: MissionId) {
        let status = self.dal.mission_status(&mission_id).await;
        if let Err(err) = status {
            log::error!("Error getting mission status: {}", err);
            return;
        }

        log::info!("Cancel mission operation");
        let mut status = status.unwrap();
        status.state = MissionUpdateState::NotActual {};
        if let Err(err) = self.dal.update_mission_status(status).await {
            log::error!("Error updating mission status: {}", err);
        }

        self.mav_mission_operation_statuses.retain(|_, status| status.id != mission_id);
        self.mission_statuses_last_sent.remove(&mission_id);
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
                return Some(protocol::send_mission_count(mav_id, total));
            },
            MissionUpdateState::Upload { total: _, progress } => {
                if progress == 0 {
                    if let Some(vehicle_id) = self.vehicle_id_from_mav_id(&mav_id) {
                        if let Ok(navigation) = self.dal.telemetry_navigation(&vehicle_id).await {
                            return Some(protocol::send_mission_home_item(mav_id, &navigation.home_position));
                        }
                    }
                    log::error!("No home position available for for MAVLink {}", mav_id);
                    return None;
                }

                let route = self.dal.mission_route(&status.id).await;
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

    pub async fn handle_mission_count(&mut self, mav_id: u8, data: &MISSION_COUNT_DATA) {
        let status = match self.mav_mission_operation_statuses.get_mut(&mav_id) {
            Some(mav_id) => mav_id,
            None => return
        };

        if let MissionUpdateState::PrepareDownload {} = status.state {
            log::info!("Got total count {} from MAVLink {}", data.count, mav_id);
            // Update status
            status.state = if data.count == 0 {
                MissionUpdateState::Actual { total: 0 }
            } else {
                MissionUpdateState::Download { total: data.count, progress: 0 }
            };

            // Crop mission items
            match self.dal.mission_route(&status.id).await {
                Ok(mut route) => {
                    if route.items.len() > data.count as usize {
                        route.items.truncate(data.count as usize);
                        if let Err(err) = self.dal.update_route(route).await {
                            log::error!("Error updating mission route: {}", err);
                        }
                    }
                },
                Err(err) => {
                    log::error!("Error getting mission route: {}", err);
                }
            }

            if let Err(err) = self.dal.update_mission_status(status.clone()).await {
                log::error!("Error updating mission status: {}", err);
            }
        }
    }

    pub async fn handle_mission_item_int(&mut self, mav_id: u8, data: &MISSION_ITEM_INT_DATA) {
        let status = match self.mav_mission_operation_statuses.get_mut(&mav_id) {
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
                let vehicle_id = match self.dal.mission_assignment(&status.id).await {
                    Ok(assignment) => assignment.vehicle_id,
                    Err(err) => {
                        log::error!("Mission {:?} is not assigned, err: {}", &status.id, err);
                        return;
                    },
                };
                let mut navigation = self.dal.telemetry_navigation(&vehicle_id).await.unwrap_or(
                    crate::models::telemetry::Navigation::default_for_id(&vehicle_id));
                navigation.home_position = protocol::mission_home_item_from_mavlink(data);
                if let Err(err) = self.dal.save_telemetry_navigation(vehicle_id, navigation).await {
                    log::error!("Save navigation telemetry error: {}", err);
                    return;
                }
            } else {
                // Add route item, zero-based index
                if let Err(err) = self.dal.upsert_route_item(
                    &status.id,
                    protocol::mission_route_item_from_mavlink(data),
                    progress - 1 // NOTE: -1 for HOME item
                ).await {
                    log::error!("Error setting mission route item: {}", err);
                }
            }

            if progress >= total {
                log::info!("Mission download completed for MAVLink {}", mav_id);
                // TODO: send ACK
                status.state = MissionUpdateState::Actual { total };
            } else {
                status.state = MissionUpdateState::Download { total, progress: progress + 1 };
            }

            if let Err(err) = self.dal.update_mission_status(status.clone()).await {
                log::error!("Error updating mission status: {}", err);
            }
        }
    }

    pub async fn handle_mission_item_request(&mut self, mav_id: u8, data: &MISSION_REQUEST_DATA) {
        let status = match self.mav_mission_operation_statuses.get_mut(&mav_id) {
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
        status.state = MissionUpdateState::Upload { total, progress: data.seq };
        if let Err(err) = self.dal.update_mission_status(status.clone()).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    pub async fn handle_mission_ack(&mut self, mav_id: u8, data: &MISSION_ACK_DATA) {
        let status = match self.mav_mission_operation_statuses.get_mut(&mav_id) {
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
        if let Err(err) = self.dal.update_mission_status(status.clone()).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    pub async fn collect_mission_messages(&mut self) -> Vec<MavMessage> {
        let mut messages = Vec::new();

        // Collect messages for active statuses
        for (mav_id, status) in self.mav_mission_operation_statuses.iter() {
            let now = tokio::time::Instant::now();
            let last_sent = self.mission_statuses_last_sent.get(&status.id);
            if last_sent.is_none() || now.duration_since(*last_sent.unwrap()) >= MISSION_RESEND_INTERVAL {
                if let Some(message) = self.process_status_to_message(mav_id, status).await {
                    messages.push(message);
                    self.mission_statuses_last_sent.insert(status.id.clone(), now);
                }
            }
        }

        // Remove unactive statuses
        self.mav_mission_operation_statuses.retain(|_, status| {
            match status.state {
                MissionUpdateState::NotActual {} |
                MissionUpdateState::Actual { .. } => false,
                _ => true,
            }
        });

        return messages;
    }
}
