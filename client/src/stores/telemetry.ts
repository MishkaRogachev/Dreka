import { writable, get, derived } from "svelte/store";

import type { Flight, Navigation, RawSns, System } from "$bindings/telemetry";

import type { WsListener } from "$datasource/ws";
import { EventsService } from "$services/events";

import { selectedVehicleId } from "$stores/vehicles";
import { nullGeodetic } from "$bindings/spatial";

export class VehicleTelemetry {
    constructor() {
        this.flight = {
            id: "",
            timestamp: 0,
            pitch: 0,
            roll: 0,
            yaw: 0,
            indicated_airspeed: 0,
            true_airspeed: 0,
            ground_speed: 0,
            throttle: 0,
            altitude_amsl: 0,
            climb: 0
        };
        this.navigation = {
            id: "",
            timestamp: 0,
            position: nullGeodetic,
            target_position: nullGeodetic,
            home_position: nullGeodetic,
            desired_pitch: 0,
            desired_roll: 0,
            desired_bearing: 0,
            target_bearing: 0,
            altitiude_error: 0,
            airspeed_error: 0,
            xtrack_error: 0,
            wp_distance: 0,
        };
        this.rawSns = {
            id: "",
            timestamp: 0,
            position: nullGeodetic,
            course: 0,
            ground_speed: 0,
            fix: 0,
            eph: 0,
            epv: 0,
            satellites_visible: 0
        };
        this.system = {
            id: "",
            timestamp: 0,
            sensors: [],
            arm_ready: false,

            battery_current: 0,
            battery_voltage: 0,
            battery_remaining: 0
        };
    }

    flight: Flight;
    navigation: Navigation;
    rawSns: RawSns;
    system: System;
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
