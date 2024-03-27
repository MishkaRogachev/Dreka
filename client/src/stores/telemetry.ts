import { writable, get, derived } from "svelte/store";

import { EventsService } from "$services/events";
import { VehicleTelemetry } from "$bindings/telemetry";
import { selectedVehicleID } from "$stores/vehicles";

export const vehiclesTelemetry = function () {
    const store = writable(new Map<string, VehicleTelemetry>(), (_, update) => {
        let listener = (data: any) => {
            let telemetry = data["telemetry"]
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
        EventsService.subscribe("TelemetryUpdated", listener);
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        telemetry: (vehicleId: string) => get(store).get(vehicleId),
        kill: () => {}
    }
} ()

export const selectedVehicleTelemetry = derived([vehiclesTelemetry, selectedVehicleID], ($data) => {
    return $data[0].get($data[1]) || new VehicleTelemetry()
})
