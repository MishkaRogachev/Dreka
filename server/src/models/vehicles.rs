use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub enum VehicleType {
    Unknown,
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
pub struct VehicleDescription {
    pub name: String,
    pub protocol_id: String,
    pub online: bool,
    pub vehicle_type: VehicleType,
    pub features: Vec<VehicleFeatures>
}
