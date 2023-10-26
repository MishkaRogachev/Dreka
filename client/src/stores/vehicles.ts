import { readable, writable, get } from 'svelte/store';

import { type VehicleDescription, type VehicleStatus, VehicleType, VehicleState } from '$bindings/vehicles';
import { VehiclesService } from '$services/vehicles';

export const vehicleDescriptions = function () {
    let interval: NodeJS.Timeout;

    const store = writable(new Map<string, VehicleDescription>(), (set, _) => {
        interval = setInterval(async () => {
            let new_vehicles = new Map<string, VehicleDescription>()
            for (const vehicle of await VehiclesService.getVehicles()) {
                if (vehicle.id) {
                    new_vehicles.set(vehicle.id, vehicle);
                }
                store.set(new_vehicles);
            }
        }, 200); // Refresh vehicle descriptions every second
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        vehicle: (vehicleId: string) => get(store).get(vehicleId),
        vehiclesIds: () => Array.from(get(store).keys()),
        vehicles: () => get(store).values(),
        saveVehicle: async (vehicle: VehicleDescription) => {
            let vehicleBack = await VehiclesService.saveVehicle(vehicle);
            store.update(vehicles => {
                if (vehicleBack && vehicleBack.id) {
                    vehicles.set(vehicleBack.id, vehicleBack);
                }
                return vehicles;
            })
            return vehicleBack;
        },
        removeVehicle: async (vehicleId: string) => {
            let vehicleIdBack = await VehiclesService.removeVehicle(vehicleId);
            store.update(vehicles => {
                if (vehicleIdBack) {
                    vehicles.delete(vehicleIdBack);
                }
                return vehicles;
            })
        },
        kill: () => clearInterval(interval)
    }
} ()

export const vehicleStatuses = function () {
    let interval: NodeJS.Timeout;

    const store = readable(new Map<string, VehicleStatus>(), (set, _) => {
        interval = setInterval(async () => {
            let new_statuses = new Map<string, VehicleStatus>()
            for (const id of vehicleDescriptions.vehiclesIds()) {
                const status = await VehiclesService.getVehicleStatus(id);
                if (status) {
                    new_statuses.set(id, status);
                }
            }
            set(new_statuses);
        }, 200);
    }); // Refresh vehicle status every 200ms

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        status: (vehicleId: string) => get(store).get(vehicleId),
        statuses: () => get(store).values(),
        kill: () => clearInterval(interval)
    }
} ()
