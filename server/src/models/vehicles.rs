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
pub struct VehicleDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(skip)]
    pub id: Option<surrealdb::sql::Thing>,
    pub name: String,
    pub protocol_id: String,
    pub online: bool,
    pub vehicle_type: VehicleType,
    pub features: Vec<VehicleFeatures>
}
