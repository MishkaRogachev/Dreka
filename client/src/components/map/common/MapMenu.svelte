<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import type { Cartesian, Geodetic } from '$bindings/spatial';

import { i18n } from '$stores/i18n';
import { selectedVehicleID } from '$stores/vehicles';
import { missions, selectedVehicleMission } from '$stores/mission';

import type { MapInteraction, MapViewport } from '$lib/interfaces/map';

import PointedMenu from '$components/common/PointedMenu.svelte';

import targetIcon from "$assets/svg/target.svg?raw";
import wptIcon from "$assets/svg/wpt.svg?raw";

export let viewport: MapViewport;
export let interaction: MapInteraction;

let isMenuOpen = false;
let menuPosition = { x: 0, y: 0 };
let clickGeodetic: Geodetic | null = null;

let clickListener = (geodetic: Geodetic, position: Cartesian) => {
    if (!selectedVehicleID || isMenuOpen) {
        isMenuOpen = false;
        return false;
    }

    menuPosition = position;
    clickGeodetic = geodetic;
    isMenuOpen = true;
    return true;
}

let viewportListener = () => {
    if (isMenuOpen && clickGeodetic) {
        menuPosition = viewport.geodeticToScreenXY(clickGeodetic);
    }
}

function closeMenu() {
    isMenuOpen = false;
}

function addWaypoint() {
    if (!$selectedVehicleMission || !clickGeodetic) {
        return;
    }
    missions.setRouteItem(
        $selectedVehicleMission.id,
        { Waypoint: { position: clickGeodetic, hold: 0, pass_radius: 0, accept_radius: 0, yaw: undefined } },
        $selectedVehicleMission.route.items.length
    );
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

<PointedMenu bind:isMenuOpen={isMenuOpen} bind:menuPosition={menuPosition}>
    <ul class="py-0">
        <li class="flex" on:click={closeMenu}>
            <div class="flex gap-x-2 items-center grow">
                { @html targetIcon }
                <a class="grow">{ $i18n.t("Guided flight here") }</a>
            </div>
        </li>
    {#if $selectedVehicleMission}
        <li class="flex" on:click={addWaypoint}>
            <div class="flex gap-x-2 items-center grow">
                { @html wptIcon }
                <a class="grow">{ $i18n.t("Add new waypoint") }</a>
            </div>
        </li>
    {/if}
    </ul>
</PointedMenu>
