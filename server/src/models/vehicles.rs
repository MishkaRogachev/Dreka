use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum VehicleType {
    Unknown,
    Auto,
    FixedWing,
    Vtol,
    RotaryWing,
    Copter
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum VehicleFeatures {
    PetrolEngine,
    Parachute,
    Lidar
}

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
#[serde_with::skip_serializing_none]
pub struct VehicleDescription {
    pub id: Option<String>,
    pub name: String,
    pub protocol_id: String,
    pub vehicle_type: VehicleType,
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
