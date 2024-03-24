import { writable, get, derived } from "svelte/store";

import { WS_TELEMETRY_URL, WsWatchdog, type WsListener } from "$datasource/ws";
import { VehicleTelemetry } from "$bindings/telemetry";
import { selectedVehicleID } from "$stores/vehicles";

export const vehiclesTelemetry = function () {
    let watchdog = new WsWatchdog(WS_TELEMETRY_URL)
    let listener: WsListener;

    const store = writable(new Map<string, VehicleTelemetry>(), (_, update) => {
        listener = (data: any) => {
            let telemetry = JSON.parse(data) as VehicleTelemetry;
            if (!telemetry) {
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
        }
        watchdog.subscribe(listener);
    });

    watchdog.start();

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        telemetry: (vehicleId: string) => get(store).get(vehicleId),
        kill: () => { watchdog.unsubscribe(listener); watchdog.stop();}
    }
} ()

export const selectedVehicleTelemetry = derived([vehiclesTelemetry, selectedVehicleID], ($data) => {
    return $data[0].get($data[1]) || new VehicleTelemetry()
})
