use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use tokio::time;
use uuid::Uuid;

use super::spatial::Geodetic;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub enum Command {
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
pub enum State {
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
pub struct Execution {
    pub id: String,
    pub vehicle_id: String,
    pub timestamp: u128,
    pub attempt: i8,
    pub state: State,
    pub command: Command
}

impl Execution {
    pub fn create(command: Command, vehicle_id: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            vehicle_id: vehicle_id.into(),
            timestamp: time::Instant::now().elapsed().as_millis(),
            attempt: 0,
            state: State::Initial,
            command
        }
    }
}
