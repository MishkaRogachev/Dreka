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

export interface MavlinkProtocolId { mav_id: number }

export type ProtocolId = {
    MavlinkId?: MavlinkProtocolId
};
export interface VehicleDescription {
    id?: string,
    name: string,
    protocol_id: ProtocolId,
    vehicle_type: VehicleType,
    features: Array<VehicleFeatures>
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

export interface VehicleStatus {
    id: string,
    last_heartbeat: number,
    state: VehicleState
}
