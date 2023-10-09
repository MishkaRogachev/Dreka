import unknownIcon from "$assets/svg/unknown.svg"
import autoIcon from "$assets/svg/auto.svg"
import fixedWingIcon from "$assets/svg/fixed_wing.svg"
import rotaryWingIcon from "$assets/svg/rotary_wing.svg"
import copterIcon from "$assets/svg/copter.svg"
import vtolIcon from "$assets/svg/vtol.svg"

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
    id: string | null,
    name: string,
    protocol_id: string,
    online: boolean, // TODO: to VehicleStatus
    vehicle_type: VehicleType,
    features: Array<VehicleFeatures>
}

export function iconFromVehicleType(vehicle_type: VehicleType): string {
    switch (vehicle_type) {
        case VehicleType.Auto:
            return autoIcon;
        case VehicleType.FixedWing:
            return fixedWingIcon;
        case VehicleType.RotaryWing:
            return rotaryWingIcon;
        case VehicleType.Copter:
            return copterIcon;
        case VehicleType.Vtol:
            return vtolIcon;
    }
    return unknownIcon;
}