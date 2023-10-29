use std::f64::consts::PI;

pub fn decode_angles(radians: f32) -> f32 {
    return radians * 180.0 / PI as f32;
}