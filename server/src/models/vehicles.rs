use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum VehicleType {
    Unknown,
    Auto,
    FixedWing,
    Vtol,
    RotaryWing,
    Copter
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
pub enum ProtocolId {
    MavlinkId { mav_id: u8 },
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
#[serde_with::skip_serializing_none]
pub struct VehicleDescription {
    pub id: Option<String>,
    pub name: String,
    pub vehicle_type: VehicleType,
    pub protocol_id: ProtocolId,
    pub features: Vec<VehicleFeatures>
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub struct VehicleStatus {
    pub id: String,
    pub is_online: bool,
}

impl VehicleStatus {
    pub fn default_for_id(link_id: &str) -> Self {
        Self {
            id: link_id.into(),
            is_online: false
        }
    }
}
