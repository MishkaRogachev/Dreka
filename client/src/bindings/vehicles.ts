import { EntityColor } from "$bindings/colors";

export enum VehicleType {
    Unknown = "Unknown",
    Auto = "Auto",
    FixedWing = "FixedWing",
    Vtol = "Vtol",
    RotaryWing = "RotaryWing",
    Copter = "Copter",
    Airship = "Airship"
}

export enum VehicleFeatures {
    PetrolEngine = "PetrolEngine",
    Parachute = "Parachute",
    Lidar = "Lidar"
}

export enum VehicleState {
    Unknown = "Unknown",
    Init = "Init",
    Boot = "Boot",
    Calibrating = "Calibrating",
    Standby = "Standby",
    Active = "Active",
    Critical = "Critical",
    Emergency = "Emergency",
    PowerOff = "PowerOff",
    FlightTermination = "FlightTermination",
}

export enum VehicleMode {
    None = "None",
    Initilaizing = "Initializing",
    Manual = "Manual",
    Acro = "Acro",
    Stabilize = "Stabilize",
    AltHold = "AltHold",
    PosHold = "PosHold",
    AltCtrl = "AltCtrl",
    PosCtrl = "PosCtrl",
    Training = "Training",
    Drift = "Drift",
    Sport = "Sport",
    Flip = "Flip",
    Break = "Break",
    Throw = "Throw",
    Follow = "Follow",
    FlowHold = "FlowHold",
    ZigZag = "ZigZag",
    FBWA = "FBWA",
    FBWB = "FBWB",
    Cruise = "Cruise",
    Autotune = "Autotune",
    Mission = "Mission",
    RTL = "RTL",
    SmartRTL = "SmartRTL",
    Circle = "Circle",
    Loiter = "Loiter",
    Orbit = "Orbit",
    Guided = "Guided",
    Takeoff = "Takeoff",
    Land = "Land",
    Avoid = "Avoid",
    Offboard = "Offboard",
    Thermal = "Thermal",
    QStabilize = "QStabilize",
    QHover = "QHover",
    QLoiter = "QLoiter",
    QLand = "QLand",
    QRTL = "QRTL",
    QAutotune = "QAutotune",
    QAcro = "QAcro"
}

export interface MavlinkProtocolId { mav_id: number }

export type ProtocolId = {
    MavlinkId?: MavlinkProtocolId
};
export interface VehicleDescription {
    id: string,
    name: string,
    color: EntityColor,
    vehicle_type: VehicleType,
    protocol_id: ProtocolId,
    features: Array<VehicleFeatures>,
    available_modes: Array<VehicleMode>,
}

export interface VehicleStatus {
    id: string,
    last_heartbeat: number,
    armed: false,
    mode: VehicleMode,
    state: VehicleState
}
