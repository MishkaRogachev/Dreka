import { type Geodetic } from "$bindings/spatial";

export interface Flight {
    id: string,
    timestamp: number,

    pitch: number,
    roll: number,
    yaw: number,

    indicated_airspeed: number,
    true_airspeed: number,
    ground_speed: number,

    throttle: number,

    altitude_amsl: number,
    climb: number,
}

export interface Navigation {
    id: string,
    timestamp: number,

    position: Geodetic,
    target_position: Geodetic,
    home_position: Geodetic,
    wp_distance: number,
    current_wp: number,
}

export interface RawSns {
    id: string,
    timestamp: number,

    position: Geodetic,
    course: number,
    ground_speed: number,
    fix: number,
    eph: number,
    epv: number,
    satellites_visible: number
}

export enum SensorType {
    Ahrs = "Ahrs",
    Accel = "Accel",
    Gyro = "Gyro",
    Mag = "Mag",
    Sns = "Sns",
    AbsPressure = "AbsPressure",
    DiffPressure = "DiffPressure",
    Laser = "Laser",
    Battery = "Battery",
    Optical = "Optical",
    Motors = "Motors",
    RadioControl = "RadioControl",
    SatComm = "SatComm",
    Avoidance = "Avoidance"
}

export interface Sensor {
    name: string,
    sensor: SensorType,
    enabled: boolean,
    health: boolean
}

export interface System {
    id: string,
    timestamp: number,

    sensors: Array<Sensor>,
    arm_ready: boolean,

    battery_current: number,
    battery_voltage: number,
    battery_remaining: number
}
