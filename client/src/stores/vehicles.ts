import { writable, derived, get } from 'svelte/store';

import { type VehicleDescription, type VehicleStatus, VehicleType } from '$bindings/vehicles';
import { VehiclesService } from '$services/vehicles';

export const IS_ONLINE_TIMEOUT = 2000;

export const selectedVecicleID = writable("")

export class Vehicle {
    constructor(description: VehicleDescription) {
        this.description = description;
    }

    is_online(): boolean {
        return Boolean(!!this.status && (Date.now() - this.status.last_heartbeat) < IS_ONLINE_TIMEOUT)
    }

    description: VehicleDescription
    status: VehicleStatus | undefined
}

export const vehicles = function () {
    let descriptionInterval: NodeJS.Timeout;
    let statusInterval: NodeJS.Timeout;

    const store = writable(new Map<string, Vehicle>(), (_, update) => {
        descriptionInterval = setInterval(async () => {
            let descriptions = await VehiclesService.getVehicleDescriptions();
            update(vehicles => {
                let usedIds = new Array<string>();

                // Add and update existing vehicles
                for (const description of descriptions) {
                    const id = description.id!;
                    usedIds.push(id);

                    if (vehicles.has(id)) {
                        vehicles.get(id)!.description = description
                    } else {
                        vehicles.set(id, new Vehicle(description))
                    }
                }

                // Delete vehicles removed by server
                for (const id of vehicles.keys()) {
                    if (!usedIds.includes(id)) {
                        vehicles.delete(id)
                    }
                }
                return vehicles;
            });
        }, 1000); // Refresh description every second

        statusInterval = setInterval(async () => {
            let statuses = await VehiclesService.getVehicleStatuses();

            update(vehicles => {
                for (const status of statuses) {
                    if (vehicles.has(status.id)) {
                        vehicles.get(status.id)!.status = status
                    }
                }
                return vehicles;
            });
        }, 250); // Refresh status every 250ms
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        vehicle: (vehicleId: string) => get(store).get(vehicleId),
        vehiclesIds: () => Array.from(get(store).keys()),
        vehicles: () => get(store).values(),
        saveVehicle: async (description: VehicleDescription) => {
            let descriptionBack = await VehiclesService.saveVehicleDescription(description);
            let vehicle: Vehicle | undefined
            if (descriptionBack && descriptionBack.id) {
                store.update(vehicles => {
                    const id = description.id!;
                    if (vehicles.has(id)) {
                        vehicles.get(id)!.description = descriptionBack!
                    } else {
                        vehicles.set(id, new Vehicle(descriptionBack!))
                    }
                    vehicle = vehicles.get(id)
                    return vehicles;
                })
            }
            console.log("----> vehicle", vehicle);
            return vehicle;
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
        kill: () => {
            clearInterval(descriptionInterval);
            clearInterval(statusInterval);
        }
    }
} ()

export const onlineVehicles = derived(vehicles, ($vehicles) => {
    return Array.from($vehicles.values()).filter(vehicle => vehicle.is_online());
})

export const vehicleTypes = [ VehicleType.Unknown, VehicleType.Auto, VehicleType.FixedWing, VehicleType.Vtol, VehicleType.RotaryWing, VehicleType.Copter, VehicleType.Airship ]

export const occupiedMavlinkIds = derived(vehicles, ($vehicles) => {
    return Array.from($vehicles.values())
        .filter(vehicle => !!vehicle.description.protocol_id.MavlinkId)
        .map(vehicle => {
        // @ts-ignore
        return vehicle.description.protocol_id.MavlinkId.mav_id;
    });
})

export const allMavlinkIds = new Array(255).fill(1).map((el, i) => i + 1)

export function getNextAvailableMavlinkId(): number | undefined {
    for (const mavId of allMavlinkIds) {
        if (get(occupiedMavlinkIds).includes(mavId))
            continue;
        return mavId;
    }
}