<script lang="ts">
import { onMount, onDestroy, createEventDispatcher } from 'svelte';

import type { Geodetic } from '$bindings/spatial';
import { VehicleMode } from '$bindings/vehicles';
import type { MissionRouteItem } from '$bindings/mission';

import { i18n } from '$stores/i18n';
import { commandExecutions } from '$stores/commands';
import { selectedVehicleId, selectedVehicle } from '$stores/vehicles';
import { formatRouteItem, missions, selectedVehicleMission } from '$stores/mission';
import { activeMapPopup } from '$stores/app';

import type { MapFacade } from '$lib/interfaces/map';

import PointedPopup from '$components/common/PointedPopup.svelte';

import playIcon from "$assets/svg/play.svg?raw";
import editIcon from "$assets/svg/edit.svg?raw";
import removeIcon from "$assets/svg/remove.svg?raw";

export let map: MapFacade;

export let routeItem: MissionRouteItem;
export let missionId: string;
export let index: number;
export let overridedPosition: Geodetic | undefined;

const dispatch = createEventDispatcher()

let gotoToken: string | null = null
let popupPosition = { x: 0, y: 0 };

$: routeItem, recalcPopupPosition();
$: gotoExecution = gotoToken ? $commandExecutions.get(gotoToken) : undefined;

$: overridedAltitude = overridedPosition &&
    Math.round(overridedPosition.altitude) !== Math.round(routeItem.position?.altitude || 0) ?
    overridedPosition.altitude : undefined;
$: overridedDistance = map.calcDistance(overridedPosition, routeItem.position) || 0;

async function gotoItem() {
    gotoToken = await commandExecutions.executeCommand(
        { SetWaypoint: { wpt: index + 1 } },
        { Vehicle: { vehicle_id: $selectedVehicleId }
    });
}

async function cancelGotoItem() {
    if (gotoToken) {
        await commandExecutions.cancelCommand(gotoToken);
    }
}

function editItem() {
    // TODO: Add waypoint editing
}

function removeItem() {
    missions.removeRouteItem(missionId, index);
    closeMenu();
}

function closeMenu() {
    dispatch('close', {});
}

function recalcPopupPosition() {
    // FIXME: home altitude
    if (overridedPosition) {
        popupPosition = map.viewport.geodeticToScreenXY(overridedPosition);
    } else if (routeItem.position) {
        popupPosition = map.viewport.geodeticToScreenXY(routeItem.position);
    } else {
        popupPosition = { x: 0, y: 0 };
    }
}

function keyListener(event: KeyboardEvent) {
    if (event.key === "Escape") {
        closeMenu();
    }
}

onMount(async () => {
    map.viewport.subscribe(recalcPopupPosition);
    document.addEventListener("keydown", keyListener);
});

onDestroy(() => {
    map.viewport.unsubscribe(recalcPopupPosition);
    document.removeEventListener("keydown", keyListener);
});

</script>
<PointedPopup 
    isPopupOpen={$activeMapPopup === "waypoint_menu"}
    popupPosition={popupPosition}>
    <p class="font-bold text-sm text-center">{ formatRouteItem(routeItem.type, index) }</p>
    <ul class="menu menu-sm p-0">
        {#if $selectedVehicle?.status?.mode == VehicleMode.Mission && $selectedVehicleMission?.status.progress.current !== index}
        <li class="flex" on:click={gotoItem}>
            <div class="flex gap-x-2 items-center grow">
                { @html playIcon }
                <a href={null} class="grow">{ $i18n.t("Goto") }</a>
            </div>
        </li>
        {/if}
        <li class="flex" on:click={editItem}>
            <div class="flex gap-x-2 items-center grow">
                { @html editIcon }
                <a href={null} class="grow">{ $i18n.t("Edit") }</a>
            </div>
        </li>
        <li class="flex" on:click={removeItem}>
            <div class="flex gap-x-2 items-center grow">
                { @html removeIcon }
                <a href={null} class="grow">{ $i18n.t("Remove") }</a>
            </div>
        </li>
    </ul>
</PointedPopup>

<PointedPopup
    isPopupOpen={$activeMapPopup === "waypoint_tooltip"}
    popupPosition={popupPosition}
    noInput={true}>
    <div class="font-bold text-sm text-center mx-2">
        {#if overridedAltitude}
        <p>{ $i18n.t("Alt") + ": " + Math.round(overridedAltitude) + " " + $i18n.t("m") }</p>
        {:else if overridedDistance > 0}
        <p>{ $i18n.t("Dist") + ": " + Math.round(overridedDistance) + " " + $i18n.t("m") }</p>
        {:else}
        <p>{ formatRouteItem(routeItem.type, index) }</p>
        {/if}
    </div>
</PointedPopup>