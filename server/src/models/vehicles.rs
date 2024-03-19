use serde::{Deserialize, Serialize};

use super::colors::EntityColor;

pub type VehicleId = String;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum VehicleType {
    Unknown,
    Auto,
    FixedWing,
    Vtol,
    RotaryWing,
    Copter,
    Airship
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum VehicleFeatures {
    PetrolEngine,
    Parachute,
    Lidar
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum VehicleState {
    Unknown,
    Init,
    Boot,
    Calibrating,
    Standby,
    Active,
    Critical,
    Emergency,
    PowerOff,
    FlightTermination
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ProtocolId {
    MavlinkId { mav_id: u8 },
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde_with::skip_serializing_none]
pub struct VehicleDescription {
    pub id: VehicleId,
    pub name: String,
    pub color: EntityColor,
    pub vehicle_type: VehicleType,
    pub protocol_id: ProtocolId,
    pub features: Vec<VehicleFeatures>
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct VehicleStatus {
    pub id: VehicleId,
    pub last_heartbeat: i64,
    pub state: VehicleState,
    pub armed: bool
}

impl VehicleStatus {
    pub fn default_for_id(vehicle_id: &VehicleId) -> Self {
        Self {
            id: vehicle_id.clone(),
            last_heartbeat: 0,
            state: VehicleState::Unknown,
            armed: false
        }
    }
}
