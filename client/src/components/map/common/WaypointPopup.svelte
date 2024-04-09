<script lang="ts">
import { onMount, onDestroy, createEventDispatcher } from 'svelte';

import type { MissionRouteItem } from '$bindings/mission';
import { missions } from '$stores/mission';
import { i18n } from '$stores/i18n';

import type { MapViewport } from '$lib/interfaces/map';

import PointedPopup from '$components/common/PointedPopup.svelte';

import editIcon from "$assets/svg/edit.svg?raw";
import removeIcon from "$assets/svg/remove.svg?raw";
// import leftIcon from "$assets/svg/left.svg?raw";
// import rightIcon from "$assets/svg/right.svg?raw";

export let viewport: MapViewport;

export let routeItem: MissionRouteItem;
export let missionId: string;
export let index: number;

let menuPosition = { x: 0, y: 0 };
let edit: boolean = false;

const dispatch = createEventDispatcher()

$: routeItem, recalcPopupPosition()

function closeMenu() {
    dispatch('close', {});
}

function switchEdit() {
    // TODO: Add waypoint editing
}

function removeItem() {
    missions.removeRouteItem(missionId, index);
    closeMenu();
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
    {#if edit}
    
        <!-- <div class="p-2 w-48 items-center"> TODO: Add waypoint editing
            <div class="join btn-sm p-0 w-full">
                <button class="btn btn-ghost btn-sm join-item" on:click={left}>{ @html leftIcon }</button>
                <button class="btn btn-ghost btn-sm join-item grow">{ index }</button>
                <button class="btn btn-ghost btn-sm join-item" on:click={right}>{ @html rightIcon }</button>
            </div>
        </div> -->
    {:else}
    <ul class="menu">
        <li class="flex" on:click={switchEdit}>
            <div class="flex gap-x-2 items-center grow">
                { @html editIcon }
                <a class="grow">{ $i18n.t("Edit waypoint") }</a>
            </div>
        </li>
        <li class="flex" on:click={removeItem}>
            <div class="flex gap-x-2 items-center grow">
                { @html removeIcon }
                <a class="grow">{ $i18n.t("Remove waypoint") }</a>
            </div>
        </li>
    </ul>
    {/if}
</PointedPopup>