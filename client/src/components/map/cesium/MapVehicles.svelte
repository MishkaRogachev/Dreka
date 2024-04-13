<script lang="ts">
import { onMount, onDestroy } from 'svelte';
import { get } from 'svelte/store';

import * as Cesium from 'cesium';

import type { MapInteractionCesium } from "$lib/map/cesium/interaction";
import { MapVehicleCesium } from "$lib/map/cesium/vehicle";

import { Vehicle, vehicles, selectedVehicleID } from "$stores/vehicles";
import { type VehicleTelemetry, vehiclesTelemetry } from "$stores/telemetry";

export let cesium: Cesium.Viewer;
export let interaction: MapInteractionCesium;

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

        // Delete vehicles removed in store
        for (const id of mapVehicles.keys()) {
            if (!usedIds.includes(id)) {
                mapVehicles.get(id)?.done();
                mapVehicles.delete(id);
            }
        }
    })

    vehiclesTelemetry.subscribe((tmi: Map<string, VehicleTelemetry>) => {
        tmi.forEach((tmi: VehicleTelemetry, vehicleID: string) => {
            let vehicle = mapVehicles.get(vehicleID);
            if (vehicle) {
                vehicle.updateFromFlight(tmi.flight);
                vehicle.updateFromNavigation(tmi.navigation);
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