<script lang="ts">
import { onMount, onDestroy, createEventDispatcher } from 'svelte';

import { VehicleMode } from '$bindings/vehicles';
import type { MissionRouteItem } from '$bindings/mission';

import { i18n } from '$stores/i18n';
import { commandExecutions } from '$stores/commands';
import { selectedVehicleId, selectedVehicle } from '$stores/vehicles';
import { formatRouteItem, missions, selectedVehicleMission } from '$stores/mission';

import type { MapViewport } from '$lib/interfaces/map';

import PointedPopup from '$components/common/PointedPopup.svelte';

import playIcon from "$assets/svg/play.svg?raw";
import editIcon from "$assets/svg/edit.svg?raw";
import removeIcon from "$assets/svg/remove.svg?raw";

export let viewport: MapViewport;

export let routeItem: MissionRouteItem;
export let missionId: string;
export let index: number;

const dispatch = createEventDispatcher()

let gotoToken: string | null = null
let menuPosition = { x: 0, y: 0 };

$: routeItem, recalcPopupPosition()
$: gotoExecution = gotoToken ? $commandExecutions.get(gotoToken) : undefined

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
    if (routeItem.position) {
        menuPosition = viewport.geodeticToScreenXY(routeItem.position);
    }
}

onMount(async () => {
    viewport.subscribe(recalcPopupPosition);

    document.addEventListener("keydown", (event: any) => {
        if (event.key === "Escape") {
            closeMenu();
        }
    });
});

onDestroy(() => {
    viewport.unsubscribe(recalcPopupPosition);
});

</script>
<PointedPopup isPopupOpen={true} bind:popupPosition={menuPosition}>
    <p class="font-bold text-sm text-center">{ formatRouteItem(routeItem.type, index) }</p>
    <ul class="menu p-0">
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