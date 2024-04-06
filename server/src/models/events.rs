use serde::{Deserialize, Serialize};

use super::communication::{LinkDescription, LinkId, LinkStatus};
use super::vehicles::{VehicleDescription, VehicleId, VehicleStatus};
use super::telemetry::VehicleTelemetry;
use super::commands::{CommandId, CommandExecution, ExecuteCommandRequest};
use super::missions::{Mission, MissionId, MissionRoute, MissionRouteItem, MissionStatus};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub enum ClientEvent {
    // Communication
    SetLinkEnabled { link_id: String, enabled: bool },

    // Commands
    ExecuteCommand { request: ExecuteCommandRequest, command_id: CommandId },
    CancelCommand { command_id: CommandId },

    // Missions
    UploadMission { mission_id: MissionId },
    DownloadMission { mission_id: MissionId },
    ClearMission { mission_id: MissionId },
    CancelMissionState { mission_id: MissionId },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub enum ServerEvent {
    // Communication
    LinkUpdated { link: LinkDescription },
    LinkRemoved { link_id: LinkId },
    LinkStatusUpdated { status: LinkStatus },

    // Vehicles
    VehicleUpdated { vehicle: VehicleDescription },
    VehicleRemoved { vehicle_id: VehicleId },
    VehicleStatusUpdated { status: VehicleStatus },

    // Telemetry
    TelemetryUpdated { telemetry: VehicleTelemetry },

    // Commands
    CommandExecutionUpdated { execution: CommandExecution },
    CommandExecutionRemoved { command_id: CommandId },

    // Missions
    MissionUpdated { mission: Mission },
    MissionRemoved { mission_id: MissionId },
    MissionStatusUpdated { status: MissionStatus },
    MissionRouteUpdated { route: MissionRoute },
    MissionRouteItemUpdated { mission_id: MissionId, index: u16, item: MissionRouteItem },
    MissionRouteItemRemoved { mission_id: MissionId, index: u16 },
}