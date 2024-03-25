use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{spatial::Geodetic, vehicles::VehicleId};

pub type CommandId = String;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub enum VehicleCommand {
    ArmDisarm { arm: bool },
    SetMode { mode: String },
    SetWaypoint { wp: u16 },

    ReturnToLaunch {},
    GoTo { wp: u32 },
    NavTo { position: Geodetic },
    SetReturn { position: Geodetic },

    SetAltitude { altitide: f32},
    SetLoiterRadius { radius: f32},

    CalibrateAirspeed {},
    CalibrateReferencePressure {},
    CalibrateTemperature {},

    SetAirSpeed { value: f32 },
    SetGroundSpeed { value: f32 },
    SetThrottle { value: u16 },
    ManualControl { pitch: f32, roll: f32, yaw: f32, thrust: u16 },
    SetServo { channel: u16, value: u16},
    OverrideServos { servos: std::collections::BTreeMap<u16, u16> },

    Takeoff { altitude: f32 },
    GoAround {}
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub enum CommandState {
    Initial,
    Sent,
    Acceped,
    Rejected,
    Denied,
    Unsupported,
    Failed,
    InProgress,
    Canceled
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub struct VehicleCommandState {
    pub id: CommandId,
    pub vehicle_id: VehicleId,
    pub command: VehicleCommand,
    pub attempt: u8,
    pub state: CommandState
}
