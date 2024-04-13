
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
    pub mav_active_statuses: HashMap<u8, MissionStatus>,
    pub waiting_ack_executions: HashMap<(u16, u8), CommandId>,

    pub executions_last_sent: HashMap<CommandId, time::Instant>,
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
            mav_active_statuses: HashMap::new(),
            waiting_ack_executions: HashMap::new(),
            executions_last_sent: HashMap::new(),
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
            MavMessage::GPS_RAW_INT(gps_raw) =>
                self.handle_gps_raw(header.system_id, gps_raw).await,
            MavMessage::SYS_STATUS(sys_data) =>
                self.handle_sys_data(header.system_id, sys_data).await,
            MavMessage::COMMAND_ACK(ack) =>
                self.handle_command_ack(header.system_id, ack).await,
            MavMessage::MISSION_COUNT(data) =>
                self.handle_mission_count(header.system_id, data).await,
            MavMessage::MISSION_ITEM_INT(data) =>
                self.handle_mission_item_int(header.system_id, data).await,
            MavMessage::MISSION_REQUEST(data) =>
                self.handle_mission_item_request(header.system_id, data).await,
            MavMessage::MISSION_ACK(ack_data) =>
                return self.handle_mission_ack(header.system_id, ack_data).await,
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
