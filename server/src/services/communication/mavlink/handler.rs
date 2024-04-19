
use std::collections::HashMap;

use tokio::{time, sync::broadcast::Receiver};
use mavlink::{MavHeader, common::MavMessage};

use crate::models::events::{ClientEvent, ServerEvent};
use crate::models::commands::CommandId;
use crate::models::vehicles::{VehicleId, VehicleMode};
use crate::models::missions::{MissionId, MissionStatus};
use crate::{bus::bus, dal::dal};

pub struct Handler {
    pub dal: dal::Dal,
    pub server_bus: bus::EventBus::<ServerEvent>,
    pub client_events_rx: Receiver<ClientEvent>,

    pub mav_vehicles: HashMap<u8, VehicleId>,
    pub mav_modes: HashMap<u8, HashMap<u32, VehicleMode>>,
    pub mav_mission_operation_statuses: HashMap<u8, MissionStatus>,
    pub waiting_ack_command_executions: HashMap<(u16, u8), CommandId>,

    pub command_executions_last_sent: HashMap<CommandId, time::Instant>,
    pub mission_statuses_last_sent: HashMap<MissionId, time::Instant>,
}

impl Handler {
    pub fn new(dal: dal::Dal, server_bus: bus::EventBus<ServerEvent>, client_events_rx: Receiver<ClientEvent>) -> Self {
        Self {
            dal,
            server_bus,
            client_events_rx,
            mav_vehicles: HashMap::new(),
            mav_modes: HashMap::new(),
            mav_mission_operation_statuses: HashMap::new(),
            waiting_ack_command_executions: HashMap::new(),
            command_executions_last_sent: HashMap::new(),
            mission_statuses_last_sent: HashMap::new()
        }
    }

    pub fn vehicle_id_from_mav_id(&self, mav_id: &u8) -> Option<VehicleId>{
        self.mav_vehicles.get(mav_id).cloned()
    }

    pub fn mav_id_from_vehicle_id(&self, vehicle_id: &VehicleId) -> Option<u8> {
        self.mav_vehicles
            .iter()
            .find(|(_, v_id)| v_id == &vehicle_id)
            .map(|(mav_id, _)| *mav_id)
    }

    pub async fn mission_id_from_mav_id(&self, mav_id: &u8) -> Option<MissionId> {
        let vehicle_id = match self.vehicle_id_from_mav_id(&mav_id) {
            Some(vehicle_id) => vehicle_id,
            None => return None
        };
        let assignment = match self.dal.mission_assignment_by_vehicle_id(&vehicle_id).await {
            Ok(assignment) => assignment,
            Err(_) => None,
        };
        if let Some(assignment) = assignment {
            Some(assignment.id)
        } else {
            None
        }
    }

    pub async fn mav_id_for_mission_id(&self, mission_id: &MissionId) -> Option<u8> {
        let assignment = self.dal.mission_assignment(&mission_id).await;
        if let Err(err) = assignment {
            log::error!("Error obtaining mission assignment: {}", err);
            return None;
        }

        match self.mav_id_from_vehicle_id(&assignment.unwrap().vehicle_id) {
            Some(mav_id) => Some(mav_id),
            None => {
                log::error!("No MAVLink vehicle for mission: {:?}", &mission_id);
                None
            }
        }
    }

    pub async fn handle_message(&mut self, header: &MavHeader, msg: &MavMessage) {
        match msg {
            MavMessage::HEARTBEAT(heartbeat_data) =>
                self.handle_heartbeat( header.system_id, heartbeat_data).await,
            MavMessage::ATTITUDE(attitude) =>
                self.handle_attitude(header.system_id, attitude).await,
            MavMessage::VFR_HUD(vfr_hud) =>
                self.handle_vfr_hud(header.system_id, vfr_hud).await,
            MavMessage::GLOBAL_POSITION_INT(global_position) =>
                self.handle_global_position(header.system_id, global_position).await,
            MavMessage::HOME_POSITION(home_position) =>
                self.handle_home_position(header.system_id, home_position).await,
            MavMessage::GPS_RAW_INT(gps_raw) =>
                self.handle_gps_raw(header.system_id, gps_raw).await,
            MavMessage::SYS_STATUS(sys_data) =>
                self.handle_sys_data(header.system_id, sys_data).await,
            MavMessage::NAV_CONTROLLER_OUTPUT(nav_data) =>
                self.handle_nav_data(header.system_id, nav_data).await,
            MavMessage::POSITION_TARGET_GLOBAL_INT(target) =>
                self.handle_target_position(header.system_id, target).await,
            MavMessage::COMMAND_ACK(ack) =>
                self.handle_command_ack(header.system_id, ack).await,
            MavMessage::MISSION_COUNT(data) =>
                self.handle_mission_count(header.system_id, data).await,
            MavMessage::MISSION_ITEM_INT(data) =>
                self.handle_mission_item_int(header.system_id, data).await,
            MavMessage::MISSION_REQUEST(data) =>
                self.handle_mission_item_request(header.system_id, data).await,
            MavMessage::MISSION_ACK(ack_data) =>
                self.handle_mission_ack(header.system_id, ack_data).await,
            MavMessage::MISSION_CURRENT(data) =>
                self.handle_mission_item_current(header.system_id, data).await,
            MavMessage::MISSION_ITEM_REACHED(data) =>
                self.handle_mission_item_reached(header.system_id, data).await,
            _ => {}
        }
    }

    async fn handle_client_event(&mut self, event: ClientEvent) {
        match event {
            ClientEvent::ExecuteCommand { request, command_id } => {
                self.add_command_execution(request, command_id).await;
            },
            ClientEvent::CancelCommand { command_id } => {
                self.cancel_command_execution(command_id).await;
            },
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

    pub async fn prepare_messages(&mut self) -> Vec<MavMessage> {
        match self.client_events_rx.try_recv() {
            Ok(event) => self.handle_client_event(event).await,
            Err(err) => {
                if err != tokio::sync::broadcast::error::TryRecvError::Empty {
                    log::error!("RX error: {}", err);
                }
            }
        }
        [self.collect_command_messages().await, self.collect_mission_messages().await].concat()
    }
}
