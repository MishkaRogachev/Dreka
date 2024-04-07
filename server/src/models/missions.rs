
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{spatial::Geodetic, vehicles::VehicleId};

pub type MissionId = String;

#[serde_as]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum MissionRouteItem {
    Gap {},
    Waypoint { position: Geodetic, hold: u16, pass_radius: f32, accept_radius: f32, yaw: Option<u16> },

    Takeoff { position: Geodetic, pitch: f32, yaw: Option<u16> },
    LandStart {},
    Landing { position: Geodetic, abort_altitude: Option<f32>, yaw: Option<u16> },

    LoiterTrn { position: Geodetic, heading_required: bool, radius: f32, turns: u16, clockwise: bool },
    LoiterAlt { position: Geodetic, heading_required: bool, radius: f32, clockwise: bool },

    TriggerCam { distance: f32, shutter: i16, trigger: bool }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct MissionRoute {
    pub id: MissionId,
    pub items: Vec<MissionRouteItem>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MissionUpdateState {
    NotActual {},
    PrepareDownload {},
    Download { total: u16, progress: u16 },
    PrepareUpload { total: u16 },
    Upload { total: u16, progress: u16 },
    Actual { total: u16 },
    Clearing {},
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MissionProgress {
    OnHold {},
    InProgress { current: u16, passed: Vec<u16> },
    Finished { passed: Vec<u16> },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MissionStatus {
    pub id: MissionId,
    pub state: MissionUpdateState,
    pub progress: MissionProgress,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct VehicleMission {
    pub id: MissionId,
    pub vehicle_id: VehicleId,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Mission {
    pub id: MissionId,
    pub vehicle_id: VehicleId,
    pub route: MissionRoute,
    pub status: MissionStatus
}
