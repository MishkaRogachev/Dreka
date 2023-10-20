
export enum VehicleType {
    Unknown = "Unknown",
    Auto = "Auto",
    FixedWing = "FixedWing",
    Vtol = "Vtol",
    RotaryWing = "RotaryWing",
    Copter = "Copter"
}

export enum VehicleFeatures {
    PetrolEngine = "PetrolEngine",
    Parachute = "Parachute",
    Lidar = "Lidar"
}

export interface VehicleDescription {
    id?: string,
    name: string,
    protocol_id: string,
    vehicle_type: VehicleType,
    features: Array<VehicleFeatures>
}

export interface VehicleStatus {
    id: string,
    is_online: boolean
}
