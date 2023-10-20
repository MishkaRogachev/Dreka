import { writable, get } from 'svelte/store';

import { type VehicleDescription, type VehicleStatus, VehicleType } from '$bindings/vehicles';
import { VehiclesService } from '$services/vehicles';

import unknownIcon from "$assets/svg/about.svg"
import autoIcon from "$assets/svg/auto.svg"
import fixedWingIcon from "$assets/svg/fixed_wing.svg"
import rotaryWingIcon from "$assets/svg/rotary_wing.svg"
import copterIcon from "$assets/svg/copter.svg"
import vtolIcon from "$assets/svg/vtol.svg"

export const all_vehicles = writable(new Map<String, VehicleDescription>())

export async function getVehicleStatus(id: string): Promise<VehicleStatus> {
    return await VehiclesService.getVehicleStatus(id) || {
        id: id,
        is_online: false,
    };
}

export async function saveVehicle(vehicle: VehicleDescription): Promise<VehicleDescription | null> {
    let vehicleBack = await VehiclesService.saveVehicle(vehicle);
    if (vehicleBack && vehicleBack.id) {
        let vehicles = get(all_vehicles);
        vehicles.set(vehicleBack.id, vehicleBack);
        all_vehicles.set(vehicles);
        return vehicleBack;
    }
    return null
}

export async function removeVehicle(vehicleId: string) {
    let vehicleIdBack = await VehiclesService.removeVehicle(vehicleId);
    let vehicles = get(all_vehicles);
    if (vehicleIdBack) {
        vehicles.delete(vehicleIdBack);
    }
    all_vehicles.set(vehicles);
}

// Refresh vehicles vehicles every second
setInterval(() => {
    VehiclesService.getVehicles().then((vehicles: Array<VehicleDescription>) => {
        let new_vehicles = new Map<String, VehicleDescription>();
        vehicles.forEach((vehicle: VehicleDescription) => {
            if (vehicle.id) {
                new_vehicles.set(vehicle.id, vehicle);
            }
        })
        all_vehicles.set(new_vehicles);
    });
}, 1000);

export function iconForVehicleType(vehicle_type: VehicleType): string {
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
