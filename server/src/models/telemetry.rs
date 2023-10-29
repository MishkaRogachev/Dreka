
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::spatial::Geodetic;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub struct FlightData {
    pub vehicle_id: String,
    pub timestamp: i64,

    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,

    pub position: Geodetic,
    pub target_position: Geodetic,

    pub indicated_airspeed: f32,
    pub true_airspeed: f32,
    pub ground_speed: f32,

    pub altitude_amsl: f32,
    pub climb: f32,

    pub wp_distance: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub struct AltitudeData {
    pub vehicle_id: String,
    pub timestamp: i64,

    pub reference_altitude: f32,
    pub altitude_amsl: f32,
    pub altitude_relative: f32,
    pub altitude_terrain: f32,
    pub bottom_clearance: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export)]
pub struct SnsData {
    pub vehicle_id: String,
    pub timestamp: i64,

    pub position: Geodetic,
    pub course: f32,
    pub ground_speed: f32,
    fix: u8,
    eph: u16,
    epv: u16,
    satellites_visible: u8,
    altitude: f32
}

impl Default for FlightData {
    fn default() -> FlightData {
        Self {
            vehicle_id: "".into(),
            timestamp: 0,

            pitch: 0.0,
            roll: 0.0,
            yaw: 0.0,

            position: Geodetic::default(),
            target_position: Geodetic::default(),

            indicated_airspeed: 0.0,
            true_airspeed: 0.0,
            ground_speed: 0.0,

            altitude_amsl: 0.0,
            climb: 0.0,

            wp_distance: 0.0,
        }
    }
}

impl Default for AltitudeData {
    fn default() -> AltitudeData {
        Self {
            vehicle_id: "".into(),
            timestamp: 0,

            reference_altitude: 0.0,
            altitude_amsl: 0.0,
            altitude_relative: 0.0,
            altitude_terrain: 0.0,
            bottom_clearance: 0.0
        }
    }
}

impl Default for SnsData {
    fn default() -> SnsData {
        Self {
            vehicle_id: "".into(),
            timestamp: 0,

            position: Geodetic::default(),
            course: 0.0,
            ground_speed: 0.0,
            fix: 0,
            eph: 0,
            epv: 0,
            satellites_visible: 0,
            altitude: 0.0
        }
    }
}