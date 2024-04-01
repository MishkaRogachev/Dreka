use serde::{Deserialize, Serialize};

use super::colors::EntityColor;

pub type VehicleId = String;
pub type PayloadId = String;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum VehicleType {
    Unknown,
    Auto,
    FixedWing,
    Vtol,
    RotaryWing,
    Copter,
    Airship
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum VehicleFeatures {
    PetrolEngine,
    Parachute,
    Lidar
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum VehicleMode {
    None,
    Initializing,
    Manual,
    Acro,
    Stabilize,
    AltHold,
    PosHold,
    AltCtrl,
    PosCtrl,
    Training,
    Drift,
    Sport,
    Flip,
    Break,
    Throw,
    Follow,
    FlowHold,
    ZigZag,
    Autorotate,
    FBWA,
    FBWB,
    Cruise,
    Autotune,
    Mission,
    RTL,
    SmartRTL,
    Circle,
    Loiter,
    Orbit,
    Guided,
    Takeoff,
    Land,
    Avoidance,
    Offboard,
    Thermal,
    QStabilize,
    QHover,
    QLoiter,
    QLand,
    QRTL,
    QAutotune,
    QAcro
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ProtocolId {
    MavlinkId { mav_id: u8 },
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde_with::skip_serializing_none]
pub struct VehicleDescription {
    pub id: VehicleId,
    pub name: String,
    pub color: EntityColor,
    pub vehicle_type: VehicleType,
    pub protocol_id: ProtocolId,
    pub features: Vec<VehicleFeatures>,
    pub available_modes: Vec<VehicleMode>
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct VehicleStatus {
    pub id: VehicleId,
    pub last_heartbeat: i64,
    pub armed: bool,
    pub mode: VehicleMode,
    pub state: VehicleState,
}

impl VehicleStatus {
    pub fn default_for_id(vehicle_id: &VehicleId) -> Self {
        Self {
            id: vehicle_id.clone(),
            last_heartbeat: 0,
            armed: false,
            state: VehicleState::Unknown,
            mode: VehicleMode::None
        }
    }
}
