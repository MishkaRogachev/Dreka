use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum VehicleClass {
    Unknown,
    Plane,
    Heli,
    Copter
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Vehicle {
    pub name: String,
    pub protocol_id: String,
    pub online: bool,
    pub class: VehicleClass
}
