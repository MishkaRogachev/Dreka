<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import { GeodeticFrame, type Cartesian, type Geodetic } from '$bindings/spatial';
import { VehicleMode } from '$bindings/vehicles';
import { MissionRouteItemType } from '$bindings/mission';

import { formatGeodeticCoordinates, i18n } from '$stores/i18n';
import { selectedVehicle, selectedVehicleId } from '$stores/vehicles';
import { selectedVehicleTelemetry } from '$stores/telemetry';
import { commandExecutions } from '$stores/commands';
import { missions, selectedVehicleMission } from '$stores/mission';
import { activeMapPopup } from '$stores/app';

import type { MapInteraction, MapViewport } from '$lib/interfaces/map';

import PointedPopup from '$components/common/PointedPopup.svelte';

import targetIcon from "$assets/svg/map_target_wpt.svg?raw";
import takeoffIcon from "$assets/svg/map_takeoff.svg?raw";
import wptIcon from "$assets/svg/map_wpt.svg?raw";
import landIcon from "$assets/svg/map_landing.svg?raw";
import copyIcon from "$assets/svg/copy.svg?raw";

export let viewport: MapViewport;
export let interaction: MapInteraction;

// TODO: to preferences
const TAKEOFF_ALTITUDE = 50;
const TAKEOFF_PITCH = 10;
const MIN_SAFE_ALTITUDE = 50;
const LAND_ALTITUDE = 20;

let menuPosition = { x: 0, y: 0 };
let clickGeodetic: Geodetic | undefined = undefined;

$: geodeticCoordinates = clickGeodetic ? formatGeodeticCoordinates(clickGeodetic).join(";") : "";
$: selectedMissonLength = $selectedVehicleMission?.route.items.length || 0;
$: hasTakeoff = $selectedVehicleMission?.route.items.some(item => item.type === MissionRouteItemType.Takeoff);
$: hasLanding = $selectedVehicleMission?.route.items.some(item => item.type === MissionRouteItemType.Landing);

let clickListener = (geodetic: Geodetic, position: Cartesian) => {
    if (!selectedVehicleId || $activeMapPopup === "map-global") {
        $activeMapPopup = "";
        return false;
    }

    if (position && geodetic) {
        menuPosition = position;
        clickGeodetic = geodetic;
        $activeMapPopup = "map-global"
        return true;
    }
    return false;
}

let viewportListener = () => {
    if ($activeMapPopup === "map-global" && clickGeodetic) {
        menuPosition = viewport.geodeticToScreenXY(clickGeodetic);
    }
}

function closeMenu() {
    $activeMapPopup = "";
    clickGeodetic = undefined;
}

async function setTarget() {
    if (!$selectedVehicleId || !clickGeodetic) {
        return;
    }

    const prevPosition = $selectedVehicleTelemetry?.navigation?.target_position || undefined;
    const position = {
        latitude: clickGeodetic.latitude,
        longitude: clickGeodetic.longitude,
        altitude: prevPosition ? prevPosition.altitude : MIN_SAFE_ALTITUDE,
        frame: prevPosition ? prevPosition.frame : GeodeticFrame.Wgs84RelativeHome
    };

    await commandExecutions.executeCommand(
        { NavTo: { position: position } },
        { Vehicle: { vehicle_id: $selectedVehicleId }
    });
    // TODO: executions handling
    closeMenu();
}

function addTakeoff() {
    if (!$selectedVehicleMission || !clickGeodetic) {
        return;
    }

    const takeoff = {
        type: MissionRouteItemType.Takeoff,
        position: {
            latitude: clickGeodetic.latitude,
            longitude: clickGeodetic.longitude,
            altitude: clickGeodetic.altitude + TAKEOFF_ALTITUDE,
            frame: GeodeticFrame.Wgs84AboveSeaLevel
        },
        pitch: TAKEOFF_PITCH,
    };

    missions.setRouteItem($selectedVehicleMission.id, takeoff, selectedMissonLength);
    closeMenu();
}

function addWaypoint() {
    if (!$selectedVehicleMission || !clickGeodetic) {
        return;
    }

    let altitude = clickGeodetic.altitude + MIN_SAFE_ALTITUDE;
    let frame = GeodeticFrame.Wgs84AboveSeaLevel;
    for (const item of [...$selectedVehicleMission.route.items].reverse()) {
        if (item.position) {
            altitude = Math.max(item.position.altitude, altitude);
            frame = item.position.frame;
            break;
        }
    }
    const waypoint = {
        type: MissionRouteItemType.Waypoint,
        position: {
            latitude: clickGeodetic.latitude,
            longitude: clickGeodetic.longitude,
            altitude: altitude,
            frame: frame
        },
        hold: 0,
        pass_radius: 0,
        accept_radius: 0
    };
    missions.setRouteItem($selectedVehicleMission.id, waypoint, selectedMissonLength);
    closeMenu();
}

function addLanding() {
    if (!$selectedVehicleMission || !clickGeodetic) {
        return;
    }

    const landing = {
        type: MissionRouteItemType.Landing,
        position: {
            latitude: clickGeodetic.latitude,
            longitude: clickGeodetic.longitude,
            altitude: clickGeodetic.altitude + LAND_ALTITUDE,
            frame: GeodeticFrame.Wgs84AboveSeaLevel
        }
    };
    missions.setRouteItem($selectedVehicleMission.id, landing, selectedMissonLength);
    closeMenu();

}

function copyCoordinates() {
    navigator.clipboard.writeText(geodeticCoordinates);
    closeMenu();
}

function keyListener(event: KeyboardEvent) {
    if (event.key === "Escape") {
        closeMenu();
    }
}

onMount(async () => {
    interaction.subscribeClick(clickListener);
    viewport.subscribe(viewportListener);

    document.addEventListener("keydown", keyListener);
});

onDestroy(() => {
    interaction.unsubscribeClick(clickListener);
    viewport.unsubscribe(viewportListener);

    document.removeEventListener("keydown", keyListener);
});

</script>

<PointedPopup isPopupOpen={$activeMapPopup === "map-global"} bind:popupPosition={menuPosition}>
    <p class="font-bold text-xs text-center mx-2">{ geodeticCoordinates }</p>
    <ul class="menu menu-sm p-0">
        {#if $selectedVehicle && $selectedVehicle.status?.mode === VehicleMode.Guided}
        <li class="flex" on:click={setTarget}>
            <div class="flex gap-x-2 items-center grow">
                { @html targetIcon }
                <a href={null} class="grow">{ $i18n.t("Target point here") }</a>
            </div>
        </li>
    {/if}
    {#if $selectedVehicleMission}
        {#if !hasTakeoff}
        <li class="flex" on:click={addTakeoff}>
            <div class="flex gap-x-2 items-center grow">
                { @html takeoffIcon }
                <a href={null} class="grow">{ $i18n.t("Add takeoff") }</a>
            </div>
        </li>
        {/if}
        <li class="flex" on:click={addWaypoint}>
            <div class="flex gap-x-2 items-center grow">
                { @html wptIcon }
                <a href={null} class="grow">{ $i18n.t("Add waypoint") }</a>
            </div>
        </li>
        {#if selectedMissonLength > 0 && !hasLanding}
        <li class="flex" on:click={addLanding}>
            <div class="flex gap-x-2 items-center grow">
                { @html landIcon }
                <a href={null} class="grow">{ $i18n.t("Add landing") }</a>
            </div>
        </li>
        {/if}
    {/if}
        <li class="flex" on:click={copyCoordinates}>
            <div class="flex gap-x-2 items-center grow">
                { @html copyIcon }
                <a href={null} class="grow">{ $i18n.t("Copy coordinates") }</a>
            </div>
        </li>
    </ul>
</PointedPopup>
