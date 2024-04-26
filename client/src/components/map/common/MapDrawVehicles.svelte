<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import type { Geodetic } from '$bindings/spatial';

import { type MapVehiclesEvent, type MapVehicles, type MapViewport } from '$lib/interfaces/map';

import { i18n } from '$stores/i18n';
import { activeMapPopup } from '$stores/app';
import { Vehicle, vehicles, selectedVehicleId } from "$stores/vehicles";
import { type VehicleTelemetry, vehiclesTelemetry } from "$stores/telemetry";
import { commandExecutions } from '$stores/commands';

import PointedPopup from '$components/common/PointedPopup.svelte';

export let mapVehicles: MapVehicles;
export let viewport: MapViewport;

let tipVehicleId: string | undefined = undefined;

$: vehicleGeodetic = tipVehicleId ? $vehiclesTelemetry.get(tipVehicleId)?.navigation?.position : undefined;
$: vehicleTargetGeodetic = tipVehicleId ? $vehiclesTelemetry.get(tipVehicleId)?.navigation?.target_position : undefined;
$: vehicleHomeGeodetic = tipVehicleId ? $vehiclesTelemetry.get(tipVehicleId)?.navigation?.home_position : undefined;

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
                if (tmi.flight) {
                    vehicle.updateFromFlight(tmi.flight);
                }
                if (tmi.navigation) {
                    vehicle.updateFromNavigation(tmi.navigation);
                }
            }
        });
    })

    selectedVehicleId.subscribe((selectedVehicleId: string) => {
        mapVehicles.setSelectedVehicle(selectedVehicleId);
    });

    mapVehicles.subscribe((event: MapVehiclesEvent) => {
        if (event.ActivateVehicle) {
            selectedVehicleId.set(event.ActivateVehicle.vehicleId);
        } else if (event.TargetPositionOrdered) {
            setTarget(event.TargetPositionOrdered.vehicleId, event.TargetPositionOrdered.position);
        } else if (event.HomePositionOrdered) {
            setHome(event.HomePositionOrdered.vehicleId, event.HomePositionOrdered.position);
        } else if (event.VehicleHovered) {
            tipVehicleId = event.VehicleHovered.hovered ? event.VehicleHovered.vehicleId : undefined;
            $activeMapPopup = "vehicle_tooltip";
        } else if (event.TargetHovered) {
            tipVehicleId = event.TargetHovered.hovered ? event.TargetHovered.vehicleId : undefined;
            $activeMapPopup = "vehicle_target_tooltip";
        } else if (event.HomeHovered) {
            tipVehicleId = event.HomeHovered.hovered ? event.HomeHovered.vehicleId : undefined;
            $activeMapPopup = "vehicle_home_tooltip";
        }
    });
});

</script>

<PointedPopup
    isPopupOpen={$activeMapPopup === "vehicle_tooltip"} noInput={true}
    popupPosition={vehicleGeodetic ? viewport.geodeticToScreenXY(vehicleGeodetic) : { x: 0, y: 0 }}>
    <div class="font-bold text-sm text-center mx-2">
        { tipVehicleId ? $vehicles.get(tipVehicleId)?.description.name : "" }
    </div>
</PointedPopup>

<PointedPopup
    isPopupOpen={$activeMapPopup === "vehicle_target_tooltip"} noInput={true}
    popupPosition={vehicleTargetGeodetic ? viewport.geodeticToScreenXY(vehicleTargetGeodetic) : { x: 0, y: 0 }}>
    <div class="font-bold text-sm text-center mx-2">
        { $i18n.t("Target position") }
    </div>
</PointedPopup>

<PointedPopup
    isPopupOpen={$activeMapPopup === "vehicle_home_tooltip"} noInput={true}
    popupPosition={vehicleHomeGeodetic ? viewport.geodeticToScreenXY(vehicleHomeGeodetic) : { x: 0, y: 0 }}>
    <div class="font-bold text-sm text-center mx-2">
        { $i18n.t("Home position") }
    </div>
</PointedPopup>