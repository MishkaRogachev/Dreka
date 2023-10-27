import { readable, writable, get } from 'svelte/store';

import { type VehicleDescription, type VehicleStatus, VehicleType } from '$bindings/vehicles';
import { VehiclesService } from '$services/vehicles';

export const vehicleDescriptions = function () {
    let interval: NodeJS.Timeout;

    const store = writable(new Map<string, VehicleDescription>(), (_, update) => {
        interval = setInterval(async () => {
            let new_vehicles = new Map<string, VehicleDescription>();
            for (const vehicle of await VehiclesService.getVehicles()) {
                if (vehicle.id) {
                    new_vehicles.set(vehicle.id, vehicle);
                }
                update(vehicles => {
                    new_vehicles.forEach((vehicle, id) => {
                        vehicles.set(id, vehicle);
                    });
                    return vehicles;
                });
            }
        }, 1000); // Refresh vehicle descriptions every second
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        vehicle: (vehicleId: string) => get(store).get(vehicleId),
        vehiclesIds: () => Array.from(get(store).keys()),
        vehicles: () => get(store).values(),
        saveVehicle: async (vehicle: VehicleDescription) => {
            let vehicleBack = await VehiclesService.saveVehicle(vehicle);
            if (vehicleBack && vehicleBack.id) {
                store.update(vehicles => {
                    // @ts-ignore
                    vehicles.set(vehicleBack.id, vehicleBack);
                    return vehicles;
                })
            }
            return vehicleBack;
        },
        removeVehicle: async (vehicleId: string) => {
            let vehicleIdBack = await VehiclesService.removeVehicle(vehicleId);
            if (vehicleIdBack) {
                store.update(vehicles => {
                    // @ts-ignore
                    vehicles.delete(vehicleIdBack);
                    return vehicles;
                })
            }
        },
        kill: () => clearInterval(interval)
    }
} ()

export const vehicleStatuses = function () {
    let interval: NodeJS.Timeout;

    const store = readable(new Map<string, VehicleStatus>(), (set, _) => {
        interval = setInterval(async () => {
            let new_statuses = new Map<string, VehicleStatus>();
            for (const status of await VehiclesService.getVehicleStatuses(vehicleDescriptions.vehiclesIds())) {
                new_statuses.set(status.id, status);
            }
            set(new_statuses);
        }, 200);
    }); // Refresh vehicles status every 200ms

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        status: (vehicleId: string) => get(store).get(vehicleId),
        statuses: () => get(store).values(),
        kill: () => clearInterval(interval)
    }
} ()

export const vehicleTypes = [ VehicleType.Unknown, VehicleType.Auto, VehicleType.FixedWing, VehicleType.Vtol, VehicleType.RotaryWing, VehicleType.Copter ]
