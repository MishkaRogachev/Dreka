<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import { GeodeticFrame, type Cartesian, type Geodetic } from '$bindings/spatial';
import { type MissionRouteItem, MissionRouteItemType } from '$bindings/mission';

import { formatGeodeticCoordinates, i18n } from '$stores/i18n';
import { selectedVehicleId } from '$stores/vehicles';
import { commandExecutions } from '$stores/commands';
import { missions, selectedVehicleMission } from '$stores/mission';
import { activeMapPopup } from '$stores/app';

import type { MapInteraction, MapViewport } from '$lib/interfaces/map';

import PointedPopup from '$components/common/PointedPopup.svelte';

import targetIcon from "$assets/svg/target.svg?raw";
import wptIcon from "$assets/svg/wpt.svg?raw";

export let viewport: MapViewport;
export let interaction: MapInteraction;

const TAKEOFF_ALTITUDE = 50;
const TAKEOFF_PITCH = 10;
const MIN_SAFE_ALTITUDE = 50;

let menuPosition = { x: 0, y: 0 };
let clickGeodetic: Geodetic | undefined = undefined;

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

    await commandExecutions.executeCommand(
        { NavTo: { position: clickGeodetic } },
        { Vehicle: { vehicle_id: $selectedVehicleId }
    });
    // TODO: executions handling
    closeMenu();
}

function addWaypoint() {
    if (!$selectedVehicleMission || !clickGeodetic) {
        return;
    }

    let newWaypoint: MissionRouteItem;
    const index = $selectedVehicleMission.route.items.length;
    if (index > 0) {
        let altitude = clickGeodetic.altitude + MIN_SAFE_ALTITUDE;
        let frame = GeodeticFrame.Wgs84AboveSeaLevel;
        for (const item of [...$selectedVehicleMission.route.items].reverse()) {
            if (item.position) {
                // TODO: correct frame handling
                altitude = Math.max(item.position.altitude, altitude);
                frame = item.position.frame;
                break;
            }
        }
        newWaypoint = {
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
    } else {
        newWaypoint = {
            type: MissionRouteItemType.Takeoff,
            position: {
                latitude: clickGeodetic.latitude,
                longitude: clickGeodetic.longitude,
                altitude: clickGeodetic.altitude + TAKEOFF_ALTITUDE,
                frame: GeodeticFrame.Wgs84AboveSeaLevel
            },
            pitch: TAKEOFF_PITCH,
        };
    }
    missions.setRouteItem($selectedVehicleMission.id, newWaypoint, index);
    closeMenu();
}

onMount(async () => {
    interaction.subscribeClick(clickListener);
    viewport.subscribe(viewportListener);

    document.addEventListener("keydown", (event: any) => {
        if (event.key === "Escape") {
            closeMenu();
        }
    });
});

onDestroy(() => {
    interaction.unsubscribeClick(clickListener);
    viewport.unsubscribe(viewportListener);
});

</script>

<PointedPopup isPopupOpen={$activeMapPopup === "map-global"} bind:popupPosition={menuPosition}>
    <p class="font-bold text-xs text-center">{ formatGeodeticCoordinates(clickGeodetic).join(";") }</p>
    <ul class="menu p-0">
        <li class="flex" on:click={setTarget}>
            <div class="flex gap-x-2 items-center grow">
                { @html targetIcon }
                <a href={null} class="grow">{ $i18n.t("Guided flight here") }</a>
            </div>
        </li>
    {#if $selectedVehicleMission}
        <li class="flex" on:click={addWaypoint}>
            <div class="flex gap-x-2 items-center grow">
                { @html wptIcon }
                <a href={null} class="grow">{ $selectedVehicleMission.route.items.length > 0 ?
                    $i18n.t("Add waypoint") : $i18n.t("Add takeoff") }</a>
            </div>
        </li>
    {/if}
    </ul>
</PointedPopup>
