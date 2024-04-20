<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import type { Geodetic } from '$bindings/spatial';

import { MapVehiclesEvent, type MapVehicles } from '$lib/interfaces/map';

import { Vehicle, vehicles, selectedVehicleId } from "$stores/vehicles";
import { type VehicleTelemetry, vehiclesTelemetry } from "$stores/telemetry";
import { commandExecutions } from '$stores/commands';

export let mapVehicles: MapVehicles;

// TODO: group all commands in a single file
async function setTarget(vehicleId: string, position: Geodetic) {
    await commandExecutions.executeCommand(
        { NavTo: { position: position } },
        { Vehicle: { vehicle_id: vehicleId }
    });
    // TODO: executions handling
}

async function setHome(vehicleId: string, position: Geodetic) {
    await commandExecutions.executeCommand(
        { SetHome: { position: position } },
        { Vehicle: { vehicle_id: vehicleId }
    });
    // TODO: executions handling
}

onMount(async () => {
    vehicles.subscribe((vehicles: Map<string, Vehicle>) => {
        let usedIds = new Array<string>();

        // Add and update existing vehicles on map
        vehicles.forEach((vehicle: Vehicle, vehicleId: string) => {
            usedIds.push(vehicleId);
            let mapVehicle = mapVehicles.vehicle(vehicleId);
            if (!mapVehicle) {
                mapVehicle = mapVehicles.addVehicle(vehicleId);
            }
            mapVehicle.updateFromDescription(vehicle.description);
            mapVehicle.updateFromStatus(vehicle.status);
        });

        // Delete vehicles removed in store
        for (const vehicleId of mapVehicles.vehicleIds()) {
            if (!usedIds.includes(vehicleId)) {
                mapVehicles.removeVehicle(vehicleId)
            }
        }
    })

    vehiclesTelemetry.subscribe((tmi: Map<string, VehicleTelemetry>) => {
        tmi.forEach((tmi: VehicleTelemetry, vehicleId: string) => {
            let vehicle = mapVehicles.vehicle(vehicleId);
            if (vehicle) {
                vehicle.updateFromFlight(tmi.flight);
                vehicle.updateFromNavigation(tmi.navigation);
            }
        });
    })

    selectedVehicleId.subscribe((selectedVehicleId: string) => {
        mapVehicles.setSelectedVehicle(selectedVehicleId);
    });

    mapVehicles.subscribe(MapVehiclesEvent.TargetChanged, (vehicleId, position: Geodetic) => {
        setTarget(vehicleId, position);
    });
    mapVehicles.subscribe(MapVehiclesEvent.HomeChanged, (vehicleId, position: Geodetic) => {
        setHome(vehicleId, position);
    });
})

onDestroy(async () => {
    mapVehicles.done();
})

</script>