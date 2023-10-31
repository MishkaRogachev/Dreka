import { type Geodetic } from "$bindings/spatial";

export interface FlightData {
    timestamp: number,
    pitch: number,
    roll: number,
    yaw: number,
    position: Geodetic,
    target_position: Geodetic,
    indicated_airspeed: number,
    true_airspeed: number,
    ground_speed: number,
    throttle: number,
    altitude_amsl: number,
    climb: number,
    wp_distance: number
}

export interface SnsData {
    timestamp: number,
    position: Geodetic,
    course: number,
    ground_speed: number,
    fix: number,
    eph: number,
    epv: number,
    satellites_visible: number
}
