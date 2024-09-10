import { writable, derived, get } from 'svelte/store';

import { type VehicleDescription, type VehicleStatus, VehicleType, VehicleMode } from '$bindings/vehicles';
import { EntityColor } from '$bindings/colors';

import type { WsListener } from '$datasource/ws';
import { ClientSideEvents, EventsService } from '$services/events';
import { VehiclesService } from '$services/vehicles';
import { i18n } from '$stores/i18n';

const ONLINE_CHECK_INTERVAL = 250;
const IS_ONLINE_TIMEOUT = 3000;

// TODO: get red of "Vehicle", use 2 strores for dscriptions and statuses
export class Vehicle {
    constructor(description: VehicleDescription) {
        this.description = description;
        this.is_online = false;
    }

    is_online: boolean

    checkIsOnline() {
        this.is_online = !!this.status && (Date.now() - this.status.last_heartbeat) < IS_ONLINE_TIMEOUT;
    }

    description: VehicleDescription
    status: VehicleStatus | undefined
}

export const vehicles = function () {
    let vehicleUpserted: WsListener;
    let vehicleRemoved: WsListener;
    let statusUpdated: WsListener;
    let wsConnected: WsListener;
    let onlineInterval: NodeJS.Timeout;

    const store = writable(new Map<string, Vehicle>(), (_, update) => {
        vehicleUpserted = (data: any) => {
            let vehicle = data["vehicle"] as VehicleDescription;
            if (!vehicle) {
                return;
            }

            update(vehicles => {
                if (vehicles.has(vehicle.id)) {
                    vehicles.get(vehicle.id)!.description = vehicle;
                } else {
                    vehicles.set(vehicle.id, new Vehicle(vehicle));
                }

                if (get(selectedVehicleId) == "") {
                    selectedVehicleId.set(vehicle.id);
                }

                return vehicles;
            });
        }

        vehicleRemoved = (data: any) => {
            let vehicle_id = data["vehicle_id"] as string;
            if (!vehicle_id) {
                return;
            }

            update(vehicles => {
                if (vehicles.has(vehicle_id)) {
                    vehicles.delete(vehicle_id);
                    if (get(selectedVehicleId) == vehicle_id) {
                        selectNextAvailableVehicle(vehicles);
                    }
                }
                return vehicles;
            });
        }

        statusUpdated = (data: any) => {
            let status = data["status"] as VehicleStatus;
            if (!status) {
                return;
            }

            update(vehicles => {
                if (vehicles.has(status.id)) {
                    vehicles.get(status.id)!.status = status;
                }
                return vehicles;
            });
        }

        wsConnected = async (_data: any) => {
            let descriptions = await VehiclesService.getVehicleDescriptions();
            if (descriptions) {
                let vehicles = new Map(descriptions!.map(description => [description.id, new Vehicle(description)]));
                for (let [id, vehicle] of vehicles) {
                    let status = await VehiclesService.getVehicleStatus(id)
                    if (status) {
                        vehicle.status = status;
                        vehicle.checkIsOnline();
                    }
                }
                selectNextAvailableVehicle(vehicles);
                update(_ => { return vehicles; });
            }
        }

        onlineInterval = setInterval(() => {
            update(vehicles => {
                for (let [_, vehicle] of vehicles) {
                    vehicle.checkIsOnline();
                }
                return vehicles;
            });

        }, ONLINE_CHECK_INTERVAL);

        EventsService.subscribe("VehicleUpserted", vehicleUpserted);
        EventsService.subscribe("VehicleRemoved", vehicleRemoved);
        EventsService.subscribe("VehicleStatusUpdated", statusUpdated);
        EventsService.subscribe(ClientSideEvents.WsConnectionOpened, wsConnected);
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
                    const id = descriptionBack!.id;
                    if (vehicles.has(id)) {
                        vehicles.get(id)!.description = descriptionBack!;
                    } else {
                        vehicles.set(id, new Vehicle(descriptionBack!));
                    }
                    vehicle = vehicles.get(id);
                    return vehicles;
                })
            }
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
            EventsService.unsubscribe("VehicleUpserted", vehicleUpserted);
            EventsService.unsubscribe("VehicleRemoved", vehicleRemoved);
            EventsService.unsubscribe("VehicleStatusUpdated", statusUpdated);
            EventsService.unsubscribe(ClientSideEvents.WsConnectionOpened, wsConnected);
            clearInterval(onlineInterval);
        }
    }
} ()

export const selectedVehicleId = writable("")

export const selectedVehicle = derived([vehicles, selectedVehicleId], ($data) => {
    return $data[0].get($data[1])
})

export const onlineVehicles = derived(vehicles, ($vehicles) => {
    return Array.from($vehicles.values()).filter(vehicle => vehicle.is_online);
})

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

export const usedVehicleTypes = [ VehicleType.Unknown, VehicleType.Auto, VehicleType.FixedWing, VehicleType.Vtol, VehicleType.RotaryWing, VehicleType.Copter, VehicleType.Airship ]
export const usedVehicleColors = [ EntityColor.Teal, EntityColor.Cyan, EntityColor.Sky, EntityColor.Blue, EntityColor.Indigo, EntityColor.Violet ]

function selectNextAvailableVehicle(vehicles: Map<string, Vehicle>) {
    let idToSelect = ""
    for (let [id, vehicle] of vehicles) {
        if (vehicle.is_online) {
            idToSelect = id;
            break;
        }
        if (!idToSelect.length) {
            idToSelect = id;
        }
    }
    selectedVehicleId.set(idToSelect);
}

export function formatMode(mode: VehicleMode | undefined) {
    if (!mode || mode == VehicleMode.None) {
        return get(i18n).t("No mode");
    }
    return get(i18n).t(mode);
}
