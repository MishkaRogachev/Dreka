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

export interface SensorData {
    name: string,
    sensor: SensorType,
    enabled: boolean,
    health: boolean
}

export interface SensorsData {
    timestamp: number,
    sensors: Array<SensorData>,
    arm_ready: boolean,
    battery_current: number,
    battery_voltage: number,
    battery_remaining: number
}
