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

export interface MavlinkProtocolId { mav_id: number }

export type ProtocolId = {
    MavlinkId?: MavlinkProtocolId
};
export interface VehicleDescription {
    id?: string,
    name: string,
    color: EntityColor,
    vehicle_type: VehicleType,
    protocol_id: ProtocolId,
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
    armed: false,
    state: VehicleState
}
