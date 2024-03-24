<script lang="ts">
import { onMount, onDestroy } from 'svelte';
import { get } from 'svelte/store';

import type { MapInteractionCesium } from "$lib/map/cesium/interaction";
import { MapVehicleCesium } from "$lib/map/cesium/vehicle";

import type { VehicleTelemetry } from "$bindings/telemetry";
import { Vehicle, vehicles, selectedVehicleID } from "$stores/vehicles";
import { vehiclesTelemetry } from "$stores/telemetry";

import * as Cesium from 'cesium';

export let interaction: MapInteractionCesium;
export let cesium: Cesium.Viewer;

let mapVehicles = new Map<string, MapVehicleCesium>

onMount(async () => {
    vehicles.subscribe((vehicles: Map<string, Vehicle>) => {
        let usedIds = new Array<string>();

        // Add and update existing vehicles on map
        vehicles.forEach((vehicle: Vehicle, vehicleID: string) => {
            usedIds.push(vehicleID);
            if (!mapVehicles.has(vehicleID)) {
                let mapVehicle = new MapVehicleCesium(cesium, interaction)
                mapVehicle.setSelected(vehicleID === get(selectedVehicleID));
                mapVehicles.set(vehicleID, mapVehicle);
            }
            mapVehicles.get(vehicleID)!.updateFromDescription(vehicle.description);
        });

        // Delete vehicles removed on server
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
                mapVehicles.get(vehicleID)!.updateFromFlight(tmi.flight);
            }
        });
    })

    selectedVehicleID.subscribe((selectedVehicleID: string) => {
        mapVehicles.forEach((vehicle: MapVehicleCesium, vehicleID: string) => {
            vehicle.setSelected(vehicleID === selectedVehicleID);
        });
    });
})

onDestroy(async () => {
    for (const id of mapVehicles.keys()) {
        mapVehicles.get(id)?.done();
        mapVehicles.delete(id);
    }
})

</script>