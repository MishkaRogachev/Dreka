export enum VehicleType {
    Unknown = "Unknown",
    Plane = "Plane",
    Vtol = "Vtol",
    Heli = "Heli",
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
