<script lang="ts">
import type { MapInteractionCesium } from "$lib/map/cesium/interaction";
import { MapVehicleCesium } from "$lib/map/cesium/vehicle";

import type { VehicleTelemetry } from "$bindings/telemetry";
import { Vehicle, vehicles } from "$stores/vehicles";
import { vehiclesTelemetry } from "$stores/telemetry";

import * as Cesium from 'cesium';

export let interaction: MapInteractionCesium;
export let cesium: Cesium.Viewer;

let mapVehicles = new Map<string, MapVehicleCesium>

vehicles.subscribe((vehicles: Map<string, Vehicle>) => {
    let usedIds = new Array<string>();

    // Add and update existing vehicles on map
    vehicles.forEach((vehicle: Vehicle, vehicleID: string) => {
        usedIds.push(vehicleID);
        if (!mapVehicles.has(vehicleID)) {
            mapVehicles.set(vehicleID, new MapVehicleCesium(cesium, interaction));
        }
        mapVehicles.get(vehicleID)!.updateFromDescription(vehicle.description);
    });

    // Delete vehicles removed by server
    for (const id of mapVehicles.keys()) {
        if (!usedIds.includes(id)) {
            mapVehicles.get(id)?.done();
            mapVehicles.delete(id);
        }
    }
})

vehiclesTelemetry.subscribe((tmi: Map<string, VehicleTelemetry>) => {
    tmi.forEach((tmi: VehicleTelemetry, vehicleID: string) => {
        if (mapVehicles.has(vehicleID) && tmi.flight) {
            mapVehicles.get(vehicleID)?.updateFromFlight(tmi.flight);
        }
    });
})

</script>