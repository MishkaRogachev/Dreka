use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{spatial::Geodetic, vehicles::{PayloadId, VehicleId, VehicleMode}};

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub enum Command {
    ArmDisarm { arm: bool },
    SetMode { mode: VehicleMode },
    SetWaypoint { wpt: u16 },
    SetHome { position: Geodetic },

    ReturnToLaunch {},
    NavTo { position: Geodetic },

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
    Initial             {},                         // Initial state
    Sent                { attempt: u8 },            // Command sent to executor
    Accepted             {},                         // Command accepted by executor
    Rejected            {},                         // Command rejected by executor
    Denied              {},                         // Command denied by executor
    Unsupported         {},                         // Command unsupported by executor or protocol
    Failed              {},                         // Command failed to execute by protocol
    InProgress          { progress: u8 },           // Command in progress by executor
    Canceled            {},                         // Command canceled by user
}
// See https://mavlink.io/en/messages/common.html#MAV_RESULT_IN_PROGRESS for more details

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub enum CommandExecutor {
    Vehicle { vehicle_id: VehicleId },
    Payload { vehicle_id: VehicleId, payload_id: PayloadId },
}

pub type CommandId = String;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub struct ExecuteCommandRequest {
    pub command: Command,
    pub executor: CommandExecutor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[derive(Clone)]
pub struct CommandExecution {
    pub id: CommandId,
    pub command: Command,
    pub executor: CommandExecutor,
    pub state: CommandState
}
