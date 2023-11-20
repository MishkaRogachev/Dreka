import { writable, get, derived } from "svelte/store";

import type { FlightData, SnsData, SensorsData } from "$bindings/telemetry";
import { nullGeodetic } from "$bindings/spatial";

import { TelemetryService } from "$services/telemetry";
import { onlineVehicles, selectedVehicleID } from "$stores/vehicles";

export class VehicleTelemetry {
    constructor() {
        this.flight = defaultFlightData;
        this.sns = defaultSnsData;
        this.sensors = defaultSensorsData;
    }

    distanceToHome(): number {
        return 0; // TODO: home & home distance
    }

    flight: FlightData
    sns: SnsData
    sensors: SensorsData
}

export const vehiclesTelemetry = function () {
    let flightInterval: NodeJS.Timeout;
    let snsInterval: NodeJS.Timeout;
    let sensorsInterval: NodeJS.Timeout;

    const store = writable(new Map<string, VehicleTelemetry>(), (_, update) => {
        flightInterval = setInterval(async () => {
            for (const vehicle of get(onlineVehicles)) {
                const vehicleID = vehicle.description.id!;
                let flightData = await TelemetryService.getVehicleFlightData(vehicleID);
                if (flightData) {
                    update(telemetry => {
                        if (!telemetry.has(vehicleID)) {
                            telemetry.set(vehicleID, new VehicleTelemetry())
                        }
                        (telemetry.get(vehicleID) as VehicleTelemetry).flight = flightData!;
                        return telemetry;
                    });
                }
            }
        }, 200); // Refresh flight every 200ms

        snsInterval = setInterval(async () => {
            for (const vehicle of get(onlineVehicles)) {
                const vehicleID = vehicle.description.id!;
                let snsData = await TelemetryService.getVehicleSnsData(vehicleID);
                if (snsData) {
                    update(telemetry => {
                        if (!telemetry.has(vehicleID)) {
                            telemetry.set(vehicleID, new VehicleTelemetry())
                        }
                        (telemetry.get(vehicleID) as VehicleTelemetry).sns = snsData!;
                        return telemetry;
                    });
                }
            }
        }, 1000); // Refresh sns every 1000ms

        sensorsInterval = setInterval(async () => {
            for (const vehicle of get(onlineVehicles)) {
                const vehicleID = vehicle.description.id!;
                let sensorsData = await TelemetryService.getVehicleSensorsData(vehicleID);
                if (sensorsData) {
                    update(telemetry => {
                        if (!telemetry.has(vehicleID)) {
                            telemetry.set(vehicleID, new VehicleTelemetry())
                        }
                        (telemetry.get(vehicleID) as VehicleTelemetry).sensors = sensorsData!;
                        return telemetry;
                    });
                }
            }
        }, 500); // Refresh sensors every 500ms
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        telemetry: (vehicleId: string) => get(store).get(vehicleId),
        kill: () => {
            clearInterval(flightInterval);
            clearInterval(snsInterval);
        }
    }
} ()

export const selectedVehicleTelemetry = derived([vehiclesTelemetry, selectedVehicleID], ($data) => {
    return $data[0].get($data[1]) || new VehicleTelemetry()
})

export const defaultFlightData: FlightData = {
    timestamp: 0,
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

export const defaultSnsData: SnsData = {
    timestamp: 0,
    position: nullGeodetic,
    course: 0,
    ground_speed: 0,
    fix: 0,
    eph: 0,
    epv: 0,
    satellites_visible: 0
}

export const defaultSensorsData: SensorsData = {
    timestamp: 0,
    sensors: [],
    arm_ready: false,
    battery_current: 0,
    battery_voltage: 0,
    battery_remaining: 0
}
