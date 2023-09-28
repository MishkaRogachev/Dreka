export enum VehicleType {
    Unknown = "Unknown",
    Plane = "Plane",
    Vtol = "Vtol",
    Heli = "Heli",
    Copter = "Copter"
}

export enum VehicleType {
    PetrolEngine = "PetrolEngine",
    Parachute = "Parachute",
    Lidar = "Lidar"
}

export interface VehicleFeatures {
    name: string,
    protocol_id: string,
    online: boolean,
    vehicle_type: VehicleType,
    features: Array<VehicleFeatures>
}
