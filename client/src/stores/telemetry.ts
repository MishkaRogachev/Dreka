import { writable, get, derived } from "svelte/store";

import type { Flight, Navigation, RawSns, System } from "$bindings/telemetry";

import type { WsListener } from "$datasource/ws";
import { EventsService } from "$services/events";

import { selectedVehicleId } from "$stores/vehicles";
import { nullGeodetic } from "$bindings/spatial";

export class VehicleTelemetry {
    flight?: Flight;
    navigation?: Navigation;
    rawSns?: RawSns;
    system?: System;
}

export const vehiclesTelemetry = function () {
    let flightUpdated: WsListener;
    let navigationUpdated: WsListener;
    let rawSnsUpdated: WsListener;
    let systemUpdated: WsListener;

    const store = writable(new Map<string, VehicleTelemetry>(), (_, update) => {
        flightUpdated = (data: any) => {
            let vehicleId = data["vehicle_id"];
            let flight = data["flight"] as Flight;
            if (!vehicleId || !flight) {
                return;
            }

            update(vehiclesTelemetry => {
                let telemetry = vehiclesTelemetry.get(vehicleId) || new VehicleTelemetry();
                telemetry.flight = flight;
                vehiclesTelemetry.set(vehicleId, telemetry);
                return vehiclesTelemetry;
            });
        }

        navigationUpdated = (data: any) => {
            let vehicleId = data["vehicle_id"];
            let navigation = data["navigation"] as Navigation;
            if (!vehicleId || !navigation) {
                return;
            }

            update(vehiclesTelemetry => {
                let telemetry = vehiclesTelemetry.get(vehicleId) || new VehicleTelemetry();
                telemetry.navigation = navigation;
                vehiclesTelemetry.set(vehicleId, telemetry);
                return vehiclesTelemetry;
            });
        }

        rawSnsUpdated = (data: any) => {
            let vehicleId = data["vehicle_id"];
            let rawSns = data["raw_sns"] as RawSns;
            if (!vehicleId || !rawSns) {
                return;
            }

            update(vehiclesTelemetry => {
                let telemetry = vehiclesTelemetry.get(vehicleId) || new VehicleTelemetry();
                telemetry.rawSns = rawSns;
                vehiclesTelemetry.set(vehicleId, telemetry);
                return vehiclesTelemetry;
            });
        }

        systemUpdated = (data: any) => {
            let vehicleId = data["vehicle_id"];
            let system = data["system"] as System;
            if (!vehicleId || !system) {
                return;
            }

            update(vehiclesTelemetry => {
                let telemetry = vehiclesTelemetry.get(vehicleId) || new VehicleTelemetry();
                telemetry.system = system;
                vehiclesTelemetry.set(vehicleId, telemetry);
                return vehiclesTelemetry;
            });
        }

        EventsService.subscribe("FlightUpdated", flightUpdated);
        EventsService.subscribe("NavigationUpdated", navigationUpdated);
        EventsService.subscribe("RawSnsUpdated", rawSnsUpdated);
        EventsService.subscribe("SystemUpdated", systemUpdated);

        // TODO: request latest telemetry for all vehicles on startup
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        telemetry: (vehicleId: string) => get(store).get(vehicleId),
        kill: () => {
            EventsService.unsubscribe("FlightUpdated", flightUpdated);
            EventsService.unsubscribe("NavigationUpdated", navigationUpdated);
            EventsService.unsubscribe("RawSnsUpdated", rawSnsUpdated);
            EventsService.unsubscribe("SystemUpdated", systemUpdated);
        }
    }
} ()

export const selectedVehicleTelemetry = derived([vehiclesTelemetry, selectedVehicleId], ($data) => {
    return $data[0].get($data[1]) || new VehicleTelemetry()
})
