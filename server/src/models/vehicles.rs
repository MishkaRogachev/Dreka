use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::colors::EntityColor;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum VehicleType {
    Unknown,
    Auto,
    FixedWing,
    Vtol,
    RotaryWing,
    Copter,
    Airship
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum VehicleFeatures {
    PetrolEngine,
    Parachute,
    Lidar
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum ProtocolId {
    MavlinkId { mav_id: u8 },
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
#[serde_with::skip_serializing_none]
pub struct VehicleDescription {
    pub id: Option<String>,
    pub name: String,
    pub color: EntityColor,
    pub vehicle_type: VehicleType,
    pub protocol_id: ProtocolId,
    pub features: Vec<VehicleFeatures>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub struct VehicleStatus {
    pub id: String,
    pub last_heartbeat: i64,
    pub state: VehicleState
}

impl VehicleStatus {
    pub fn default_for_id(link_id: &str) -> Self {
        Self {
            id: link_id.into(),
            last_heartbeat: 0,
            state: VehicleState::Unknown
        }
    }
}
