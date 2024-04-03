use std::f64::consts::PI;

pub fn decode_angles(radians: f32) -> f32 {
    return radians * 180.0 / PI as f32;
}

pub fn decode_lat_lon(value: i32) -> f64 {
    return value as f64 / 1e7;
}

// pub fn encode_lat_lon(value: f64) -> i32 {
//     return (value * 1e7) as i32;
// }

pub fn decode_altitude(value: i32) -> f32 {
    return value as f32 / 1000.0;
}

pub fn decode_cog_or_hdg(value: u16) -> f32 {
    return value as f32 / 100.0;
}

pub fn decode_ground_speed(value: u16) -> f32 {
    return value as f32 / 100.0;
}

pub fn to_true_airspeed(ias: f32, altitude: f32) -> f32 {
    return ias + (ias * 0.02 * altitude / 1000.0);
}

pub fn decode_voltage(value: u16) -> f32 {
    return value as f32 / 1000.0;
}

pub fn decode_current(value: i16) -> f32 {
    return value as f32 / 100.0;
}
