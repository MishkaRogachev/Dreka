
use serde::{Deserialize, Serialize};

use super::spatial::Geodetic;

pub type TelemetryId = String;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum SensorType {
    Ahrs,
    Accel,
    Gyro,
    Mag,
    Sns,
    AbsPressure,
    DiffPressure,
    Laser,
    Battery,
    Optical,
    Motors,
    RadioControl,
    SatComm,
    Avoidance
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Flight {
    pub id: TelemetryId,
    pub timestamp: i64,

    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,

    pub indicated_airspeed: f32,
    pub true_airspeed: f32,
    pub ground_speed: f32,

    pub throttle: u16,

    pub altitude_amsl: f32,
    pub climb: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Navigation {
    pub id: TelemetryId,
    pub timestamp: i64,

    pub position: Geodetic,
    pub target_position: Geodetic,
    pub home_position: Geodetic,
    pub wp_distance: f32,
    pub current_wp: u16,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct RawSns {
    pub id: TelemetryId,
    pub timestamp: i64,

    pub position: Geodetic,
    pub course: f32,
    pub ground_speed: f32,
    pub fix: u8,
    pub eph: u16,
    pub epv: u16,
    pub satellites_visible: u8
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Sensor {
    pub name: String,
    pub sensor: SensorType,
    pub enabled: bool,
    pub health: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct System {
    pub id: TelemetryId,
    pub timestamp: i64,

    pub sensors: Vec<Sensor>,
    pub arm_ready: bool,

    pub battery_current: f32,
    pub battery_voltage: f32,
    pub battery_remaining: i8
}

impl Flight {
    pub fn default_for_id(id: &TelemetryId) -> Self {
        Self {
            id: id.clone(),
            timestamp: 0,
            pitch: 0.0,
            roll: 0.0,
            yaw: 0.0,
            indicated_airspeed: 0.0,
            true_airspeed: 0.0,
            ground_speed: 0.0,
            throttle: 0,
            altitude_amsl: 0.0,
            climb: 0.0
        }
    }
}

impl Navigation {
    pub fn default_for_id(id: &TelemetryId) -> Self {
        Self {
            id: id.clone(),
            timestamp: 0,
            position: Geodetic::default(),
            target_position: Geodetic::default(),
            home_position: Geodetic::default(),
            wp_distance: 0.0,
            current_wp: 0
        }
    }
}

impl RawSns {
    pub fn default_for_id(id: &TelemetryId) -> Self {
        Self {
            id: id.clone(),
            timestamp: 0,
            position: Geodetic::default(),
            course: 0.0,
            ground_speed: 0.0,
            fix: 0,
            eph: 0,
            epv: 0,
            satellites_visible: 0
        }
    }
}

impl System {
    pub fn default_for_id(id: &TelemetryId) -> Self {
        Self {
            id: id.clone(),
            timestamp: 0,
            sensors: Vec::new(),
            arm_ready: false,
            battery_current: 0.0,
            battery_voltage: 0.0,
            battery_remaining: 0
        }
    }
}