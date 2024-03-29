use serde::{Deserialize, Serialize};

use super::communication::{LinkDescription, LinkId, LinkStatus};
use super::vehicles::{VehicleDescription, VehicleId, VehicleStatus};
use super::telemetry::VehicleTelemetry;
use super::commands::{CommandId, VehicleCommand, VehicleCommandState};

// TODO: consider to move events to an event bus
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub enum ClientEvent {
    // Communication
    SetLinkEnabled { link_id: String, enabled: bool },
    // Commands
    ExecuteCommand { vehicle_id: VehicleId, command: VehicleCommand },
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
    VehicleRemoved { vehicle_id: String },
    VehicleStatusUpdated { status: VehicleStatus },

    // Telemetry
    TelemetryUpdated { telemetry: VehicleTelemetry },

    // Commands
    CommandUpdated { command: VehicleCommandState },
    CommandRemoved { command_id: CommandId },
}