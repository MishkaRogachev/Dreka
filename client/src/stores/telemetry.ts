import { writable, get, derived } from "svelte/store";

import { VehicleTelemetry, type Flight, type Navigation, type System } from "$bindings/telemetry";
import { nullGeodetic } from "$bindings/spatial";
import { ws_telemetry } from "$datasource/ws";
import { onlineVehicles, selectedVehicleID } from "$stores/vehicles";

export const vehiclesTelemetry = function () {
    // TODO: move socket managment to telemetry service
    let requestWsInterval: NodeJS.Timeout;
    let ws: WebSocket | null = null;

    const store = writable(new Map<string, VehicleTelemetry>(), (_, update) => {
        requestWsInterval = setInterval(async () => {
            if (!ws) {
                ws = ws_telemetry();

                ws.addEventListener("open", event => {
                    console.log("WebSocket connection established");
                });

                ws.addEventListener("message", event => {
                    let telemetry: VehicleTelemetry = JSON.parse(event.data);
                    if (!telemetry) {
                        console.log("Error parsing telemetry: ", event.data);
                        return;
                    }

                    update(vehiclesTelemetry => {
                        if (vehiclesTelemetry.has(telemetry.vehicle_id)) {
                            let actualTelemetry = vehiclesTelemetry.get(telemetry.vehicle_id)!;
                            if (telemetry.timestamp > actualTelemetry.timestamp) {
                                if (telemetry.flight) {
                                    actualTelemetry.flight = telemetry.flight;
                                }
                                if (telemetry.navigation) {
                                    actualTelemetry.navigation = telemetry.navigation;
                                }
                                if (telemetry.system) {
                                    actualTelemetry.system = telemetry.system;
                                }
                            }
                        } else {
                            vehiclesTelemetry.set(telemetry.vehicle_id, telemetry);
                        }
                        return vehiclesTelemetry;
                    });
                });

                ws.addEventListener("close", event => { ws = null; });
                ws.addEventListener("error", event => { ws = null; });
            }

        }, 1000); // Check websocket every second
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        telemetry: (vehicleId: string) => get(store).get(vehicleId),
        kill: () => {
            if (ws) {
                ws.close();
                ws = null;
            }
        }
    }
} ()

export const selectedVehicleTelemetry = derived([vehiclesTelemetry, selectedVehicleID], ($data) => {
    return $data[0].get($data[1]) || new VehicleTelemetry(get(selectedVehicleID))
})

export const defaultFlight: Flight = {
    pitch: 0,
    roll: 0,
    yaw: 0,
    position: nullGeodetic,
    target_position: nullGeodetic,
    indicated_airspeed: 0,
    true_airspeed: 0,
    ground_speed: 0,
    throttle: 0,
    altitude_amsl: 0,
    climb: 0,
    wp_distance: 0
}

export const defaultNavigation: Navigation = {
    position: nullGeodetic,
    course: 0,
    ground_speed: 0,
    fix: 0,
    eph: 0,
    epv: 0,
    satellites_visible: 0
}

export const defaultSystem: System = {
    sensors: [],
    arm_ready: false,
    battery_current: 0,
    battery_voltage: 0,
    battery_remaining: 0
}
