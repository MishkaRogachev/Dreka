<script lang="ts">
import {  onDestroy } from 'svelte';

import { i18n } from "$stores/i18n";
import { mainMap } from "$stores/app";
import { formatRouteItem, missions } from "$stores/mission";

import BaseDialog from "$components/dialogs/BaseDialog.svelte";
import PositionEdit from "$components/common/PositionEdit.svelte";

import leftIcon from "$assets/svg/left.svg?raw";
import rightIcon from "$assets/svg/right.svg?raw";
import centerIcon from "$assets/svg/center.svg?raw";

export let disabled: boolean = false;
export let missionId: string;
export let index: number;
let closeDialog: () => void;

$: route = $missions.get(missionId)?.route
$: routeItem = route ? route.items.at(index) : undefined

$: {
    if (closeDialog && (!route || index < 0 || index >= route?.items.length)) {
        closeDialog();
    }
}

$: {
    $mainMap?.missions.mission(missionId)?.highlightRouteItem(index);
}

function centerOnMap() {
    $mainMap?.missions.mission(missionId)?.centerOnMap(index);
}

function left() {
    index--;
}

function right() {
    index++;
}

onDestroy(() => {
    $mainMap?.missions.mission(missionId)?.highlightRouteItem(-1);
});

</script>

<BaseDialog bind:closeDialog={closeDialog}>
    <div slot="title" class="flex gap-2 items-center">
        <button class="btn btn-sm btn-circle px-1 btn-ghost" disabled={!route || index < 1} on:click={left}>
            {@html leftIcon}
        </button>
        <button class="btn btn-sm btn-circle px-1 btn-ghost" disabled={!route || index > route.items.length - 2} on:click={right}>
            {@html rightIcon}
        </button>
        <p class="w-full">{ routeItem ? formatRouteItem(routeItem.type, index) : $i18n.t("No item") }</p>
        <button class="btn btn-sm btn-circle px-1 btn-ghost" disabled={!routeItem || !routeItem.position} on:click={centerOnMap}>
            {@html centerIcon}
        </button>
    </div>
    <div slot="content" class="grid grid-cols-2 gap-1 w-80">
    <!-- TODO: select for route item type -->
    {#if routeItem && routeItem.position}
        <PositionEdit bind:position={routeItem.position} disabled={disabled}/>
    {/if}
    <!-- WAYPOINT -->
    {#if routeItem && routeItem.pass_radius !== undefined}
        <h1 class="font-medium my-2 w-full">{ $i18n.t("Pass radius") + ", " + "m" }</h1>
        <input disabled={disabled} type="number" step=".01" min="0" placeholder={ $i18n.t("Default") } class="input input-sm w-full"
        bind:value={routeItem.pass_radius}/>
    {/if}
    {#if routeItem && routeItem.accept_radius !== undefined}
        <h1 class="font-medium my-2 w-full">{ $i18n.t("Accept radius") + ", " + "m" }</h1>
        <input disabled={disabled} type="number" step=".01" min="0" placeholder={ $i18n.t("Default") } class="input input-sm w-full"
        bind:value={routeItem.accept_radius}/>
    {/if}
    {#if routeItem && routeItem.yaw !== undefined}
        <h1 class="font-medium my-2 w-full">{ $i18n.t("Yaw") + ", " + "m" }</h1>
        <input disabled={disabled} type="number" step="1" min="0" max="359" placeholder={ $i18n.t("Default") } class="input input-sm w-full"
        bind:value={routeItem.yaw}/>
    {/if}
    {#if routeItem && routeItem.hold !== undefined}
        <h1 class="font-medium my-2 w-full">{ $i18n.t("Hold") + ", " + "s" }</h1>
        <input disabled={disabled} type="number" step="1" min="0" placeholder={ $i18n.t("Default") } class="input input-sm w-full"
        bind:value={routeItem.hold}/>
    {/if}
    <!-- TAKEOFF -->
    {#if routeItem && routeItem.pitch !== undefined}
        <h1 class="font-medium my-2 w-full">{ $i18n.t("Pitch") + ", " + "m" }</h1>
        <input disabled={disabled} type="number" step="1" min="0" max="90" placeholder={ $i18n.t("Default") } class="input input-sm w-full"
        bind:value={routeItem.pitch}/>
    {/if}
    <!-- LAND -->
    {#if routeItem && routeItem.abort_altitude !== undefined}
        <h1 class="font-medium my-2 w-full">{ $i18n.t("Abort Altitude") + ", " + $i18n.t("m") }</h1>
        <input disabled={disabled} type="number" step=".01" min="-500" max="50000"
        placeholder={ $i18n.t("Default") } class="input input-sm w-full"
        bind:value={routeItem.abort_altitude}/>
    {/if}
    <!-- LOITER -->
    {#if routeItem && routeItem.radius !== undefined}
        <h1 class="font-medium my-2 w-full">{ $i18n.t("Radius") + ", " + "m" }</h1>
        <input disabled={disabled} type="number" step=".01" min="0" placeholder={ $i18n.t("Default") } class="input input-sm w-full"
        bind:value={routeItem.radius}/>
    {/if}
    {#if routeItem && routeItem.turns !== undefined}
        <h1 class="font-medium my-2 w-full">{ $i18n.t("Turns") + ", " + "cnt" }</h1>
        <input disabled={disabled} type="number" step="1" min="0" placeholder={ $i18n.t("Default") } class="input input-sm w-full"
        bind:value={routeItem.turns}/>
    {/if}
    {#if routeItem && routeItem.clockwise !== undefined}
        <h1 class="font-medium my-2 w-full">{ $i18n.t("Clockwise") + ", " + $i18n.t("m") }</h1>
        <input disabled={disabled} type="checkbox" class="checkbox checkbox-sm"
        bind:checked={routeItem.clockwise}/>
    {/if}
    {#if routeItem && routeItem.heading_required !== undefined}
        <h1 class="font-medium my-2 w-full">{ $i18n.t("Heading required") + ", " + $i18n.t("m") }</h1>
        <input disabled={disabled} type="checkbox" class="checkbox checkbox-sm"
        bind:checked={routeItem.heading_required}/>
    {/if}
    <!-- TODO: other parameters
    // distance?: number;
    // shutter?: number;
    // trigger?: boolean; -->
    </div>
</BaseDialog>
