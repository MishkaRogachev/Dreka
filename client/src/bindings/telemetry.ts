import { type Geodetic } from "$bindings/spatial";

export interface Flight {
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

export interface Navigation {
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
    sensors: Array<Sensor>,
    arm_ready: boolean,
    battery_current: number,
    battery_voltage: number,
    battery_remaining: number
}

export class VehicleTelemetry {
    constructor() {
        this.vehicle_id = "";
        this.timestamp = Date.now();
        this.flight = undefined;
        this.navigation = undefined;
        this.system = undefined;
    }

    vehicle_id: string
    timestamp: number
    flight?: Flight
    navigation?: Navigation
    system?: System
}