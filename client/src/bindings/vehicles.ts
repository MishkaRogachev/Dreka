export enum VehicleType {
    Unknown = "Unknown",
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
    name: string,
    protocol_id: string,
    online: boolean,
    vehicle_type: VehicleType,
    features: Array<VehicleFeatures>
}
