<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import type { Geodetic } from '$bindings/spatial';

import { type MapVehiclesEvent, type MapVehicles, type MapViewport } from '$lib/interfaces/map';

import { Vehicle, vehicles, selectedVehicleId } from "$stores/vehicles";
import { type VehicleTelemetry, vehiclesTelemetry } from "$stores/telemetry";
import { commandExecutions } from '$stores/commands';

import PointedPopup from '$components/common/PointedPopup.svelte';

export let mapVehicles: MapVehicles;
export let viewport: MapViewport;

let tipVehicleId: string | undefined = undefined;

$: vehicleTipGeodetic = tipVehicleId ? $vehiclesTelemetry.get(tipVehicleId)?.navigation?.position : undefined;
$: vehicleTipPosition = vehicleTipGeodetic ? viewport.geodeticToScreenXY(vehicleTipGeodetic) : { x: 0, y: 0 };

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

    mapVehicles.subscribe((event: MapVehiclesEvent) => {
        if (event.Activated) {
            selectedVehicleId.set(event.Activated.vehicleId);
        } else if (event.TargetPositionOrdered) {
            setTarget(event.TargetPositionOrdered.vehicleId, event.TargetPositionOrdered.position);
        } else if (event.HomePositionOrdered) {
            setHome(event.HomePositionOrdered.vehicleId, event.HomePositionOrdered.position);
        } else if (event.Hovered) {
            tipVehicleId = event.Hovered.hovered ? event.Hovered.vehicleId : undefined;
        }
    });
})

onDestroy(async () => {
    mapVehicles.done();
})

</script>

<PointedPopup isPopupOpen={!!tipVehicleId} bind:popupPosition={vehicleTipPosition}>
    <div class="font-bold text-sm text-center mx-2">
        { tipVehicleId ? $vehicles.get(tipVehicleId)?.description.name : "" }
    </div>
</PointedPopup>