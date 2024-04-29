<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import MapLayersView from './MapLayersView.svelte';

import { longpress } from "$lib/common/longpress";
import { roundTo125 } from "$lib/common/formats";
import type { MapFacade } from "$lib/interfaces/map";

import { userPreferences } from '$stores/preferences';
import { formatGeodeticCoordinates, i18n } from '$stores/i18n';
import { activeMapPopup } from '$stores/app';

import crossImg from "$assets/svg/map_cross.svg";

import compasIcon from "$assets/svg/compas.svg?raw";
import layersIcon from "$assets/svg/layers.svg?raw"
import crossIcon from "$assets/svg/cross_mode.svg?raw";
import minusIcon from "$assets/svg/minus.svg?raw";
import plusIcon from "$assets/svg/plus.svg?raw";
import rulerIcon from "$assets/svg/ruler.svg?raw";
import closeIcon from "$assets/svg/close.svg?raw";
import gridIcon from "$assets/svg/grid.svg?raw";

export let map: MapFacade;

const scaleFactor: number = 50

let scaleWidth: number = 0.0;

let heading: number = 0.0;
let pixelScale: number = 0.0;
let geodeticCoordinates: string;
let crossMode: boolean = false;

let rulerMode: boolean = false;
let rulerLength: number = 0.0;
let gridMode: boolean = false;

let mouseInterval: NodeJS.Timeout;

$: metersInWidth = pixelScale * scaleWidth;
$: metersRounded = roundTo125(metersInWidth);

onMount(async () => {
    map.viewport.subscribe(viewportListener);
    mouseInterval = setInterval(() => {
        if (!crossMode) {
            let geodetic = map.viewport.screenXYToGeodetic(map.interaction.mouseCoordinates());
            geodeticCoordinates = formatGeodeticCoordinates(geodetic).join(";");
        }

        pixelScale = map.viewport.pixelScale();
        rulerLength = Math.round(map.ruler.distance());
    }, 100);
    viewportListener();
});

onDestroy(() => {
    map.viewport.unsubscribe(viewportListener);
    clearInterval(mouseInterval);
});

let viewportListener = () => {
    heading = map.viewport.heading();
    pixelScale = map.viewport.pixelScale();

    let geodetic = crossMode ?
        map.viewport.screenXYToGeodetic({ x: map.viewport.viewportWidth() / 2, y: map.viewport.viewportHeight() / 2 }) :
        map.viewport.screenXYToGeodetic(map.interaction.mouseCoordinates());
    geodeticCoordinates = formatGeodeticCoordinates(geodetic).join(";");

    $userPreferences.set("map/viewport", JSON.stringify(map.viewport.save()));
}

function resetCompas() { map.viewport.resetView(); }
function switchCrossMode() { crossMode = !crossMode; }
function coordsToClipboard() { navigator.clipboard.writeText(geodeticCoordinates); }
function zoomIn() { map.viewport.zoomIn(pixelScale * scaleFactor); }
function zoomOut() { map.viewport.zoomOut(pixelScale * scaleFactor); }

function switchRulerMode() {
    $activeMapPopup = "";
    rulerMode = !rulerMode;
    map.ruler.setEnabled(rulerMode);
}

function switchGridMode() {
    gridMode = !gridMode;
    map.graticule.setEnabled(gridMode);
}

function clearRuler() { map.ruler.clear(); }
</script>

<style>
#mapControlPanel {
    position: absolute;
    width: 60%;
    bottom: 42px;
    left: 12px;
    gap: 4px;
    background: transparent;
    display: flex;
    flex-direction: row;
    align-items: center;
}

#cross-aim {
    position: absolute;
    top: 50%;
    left: 50%;
}
#scale {
    width: 128px;
    border-bottom: 3px solid currentColor;
}

.scale-tick {
    position: absolute;
    border-left: 3px solid currentColor;
    height: 8px;
    bottom: 0px;
}

#ruler {
    width: 96px;
    display: flex;
    justify-content: center;
    align-items: center;
}

</style>

<img id=cross-aim hidden={!crossMode} src={crossImg} class="pointer-events-none"/>

<div id="mapControlPanel">
    <!-- Compass -->
    <div class="tooltip" data-tip={ $i18n.t("To North") }>
        <button class="btn btn-sm btn-circle scale-125" on:click={resetCompas}>
            <div style="transform:rotate({heading}deg);">{@html compasIcon}</div>
        </button>
    </div>

    <!-- Coordinates -->
    <div class="join btn-sm p-0" >
        <div class="tooltip" data-tip={ crossMode ? $i18n.t("Disable cross mode") : $i18n.t("Enable cross mode") }>
            <button class={ "btn btn-sm px-1 ml-2 " + (crossMode ? "btn-accent join-item" : "") } on:click={switchCrossMode}>
                {@html crossIcon}
            </button>
        </div>
        {#if crossMode}
        <div class="tooltip" data-tip={ $i18n.t("Copy to clipboard") }>
        <button class="btn btn-sm join-item font-mono flex-nowrap w-64" on:click={coordsToClipboard}>
            <a href={null}>{ geodeticCoordinates }</a>
        </button>
        </div>
        {/if}
    </div>

    <!-- Map scale -->
    <div class="join btn-sm p-0">
        <div class="tooltip" data-tip={ $i18n.t("Zoom out") }>
            <button class="btn btn-sm px-1 join-item"
                use:longpress={{ delay: 100, repeat: true, onLongPress: zoomOut, onIdleClick: zoomOut }}>
                {@html minusIcon}
            </button>
        </div>
        <div id="scale" class="btn-sm bg-base-200 px-1 join-item" bind:clientWidth={scaleWidth}>
            {metersRounded > 1000 ? (metersRounded / 1000 + $i18n.t(" km")) : (metersRounded + " m")}
            <div class="scale-tick" style ="left: 0%"></div>
            <div class="scale-tick" style ="left: {metersRounded / metersInWidth * 100}%"></div>
        </div>
        <div class="tooltip" data-tip={ $i18n.t("Zoom in") }>
            <button class="btn btn-sm px-1 join-item"
                use:longpress={{ delay: 100, repeat: true, onLongPress: zoomIn, onIdleClick: zoomIn }}>
                {@html plusIcon}
            </button>
        </div>
    </div>

    <!-- Ruler Tool -->
    <div class="join btn-sm p-0">
        <div class="tooltip" data-tip={ rulerMode ? $i18n.t("Disable ruler") : $i18n.t("Enable ruler") }>
            <button class={"btn btn-sm px-2 " + (rulerMode ? "btn-accent " : "") + (rulerLength > 0 ? "join-item" : "")}
                on:click={switchRulerMode}>
                {@html rulerIcon }
            </button>
        </div>
        {#if rulerLength > 0}
        <div id="ruler" class="btn-sm bg-base-200 px-1 join-item">
            {rulerLength > 1000 ? ((Math.round(rulerLength / 100) / 10).toString() + $i18n.t(" km")) : (rulerLength + $i18n.t(" m"))}
        </div>
        <div class="tooltip" data-tip={ $i18n.t("Clear ruler") }>
            <button class="btn btn-sm px-1 join-item" on:click={clearRuler}>
                {@html closeIcon}
            </button>
        </div>
        {/if}
    </div>

    <!-- Grid Tool -->
    <div class="join btn-sm p-0">
        <div class="tooltip" data-tip={ gridMode ? $i18n.t("Disable grid") : $i18n.t("Enable grid") }>
            <button class={"btn btn-sm px-2 " + (gridMode ? "btn-accent" : "")} on:click={switchGridMode}>{@html gridIcon}</button>
        </div>
    </div>

    <!-- Map Layers -->
    <!-- TODO: dropdown button -->
    <div class="join btn-sm p-0">
        <div class="tooltip" data-tip={ $i18n.t("Map layers") }>
            <div tabindex="0" class="dropdown dropdown-top dropdown-end">
                <label tabindex="0" class="btn btn-sm px-2">{@html layersIcon}</label>
                <ul class="dropdown-content z-[1] menu p-2 my-2 shadow bg-base-100 rounded-box">
                    <MapLayersView layers={map.layers}/>
                </ul>
            </div>
        </div>
    </div>
</div>
