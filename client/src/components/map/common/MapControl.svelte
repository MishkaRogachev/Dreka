<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import MapLayersView from './MapLayersView.svelte';

import { degreesToDmsString, roundTo125 } from "$lib/common/formats";
import type { MapViewport, MapInteraction, MapRuler, MapGraticule, MapLayers } from "$lib/interfaces/map";

import crossImg from "$assets/svg/cross.svg";

import compasIcon from "$assets/svg/compas.svg?raw";
import layersIcon from "$assets/svg/layers.svg?raw"
import crossIcon from "$assets/svg/cross.svg?raw";
import cursorIcon from "$assets/svg/cursor.svg?raw";
import minusIcon from "$assets/svg/minus.svg?raw";
import plusIcon from "$assets/svg/plus.svg?raw";
import rulerIcon from "$assets/svg/ruler.svg?raw";
import closeIcon from "$assets/svg/close.svg?raw";
import gridIcon from "$assets/svg/grid.svg?raw";

export let viewport: MapViewport;
export let interaction: MapInteraction;
export let ruler: MapRuler;
export let graticule: MapGraticule;
export let layers: MapLayers;

const scaleFactor: number = 10

let scaleWidth: number = 0.0;
let zoomInPressed: boolean = false;
let zoomOutPressed: boolean = false;

let heading: number = 0.0;

let pixelScale: number = 0.0;
let metersInWidth: number = 0.0;
let metersRounded: number = 0.0;

let latitude: string = "-";
let longitude: string = "-";

let crossMode: boolean = false;

let rulerMode: boolean = false;
let rulerLength: number = 0.0;

let gridMode: boolean = false;

let interval: any;

onMount(async () => {
    // Update UI every 50ms
    interval = setInterval(() => {
        heading = viewport.heading();
        pixelScale = viewport.pixelScale();
        metersInWidth = pixelScale * scaleWidth;
        metersRounded = roundTo125(metersInWidth);

        let geodetic = crossMode ? viewport.screenXYToGeodetic({ x: viewport.viewportWidth() / 2, 
                                                                    y: viewport.viewportHeight() / 2 }) :
                                    viewport.screenXYToGeodetic(interaction.mouseCoordinates());
        latitude = degreesToDmsString(geodetic.latitude, false);
        longitude = degreesToDmsString(geodetic.longitude, true);

        rulerLength = Math.round(ruler.distance());

        if (zoomInPressed) {
            viewport.zoomIn(pixelScale * scaleFactor);
        }

        if (zoomOutPressed) {
            viewport.zoomOut(pixelScale * scaleFactor);
        }
    }, 50);
})

onDestroy(async () => { clearInterval(interval); })

function resetCompas() { viewport.lookTo(0, -90, 2); }
function switchCrossMode() { crossMode = !crossMode; }
function coordsToClipboard() { navigator.clipboard.writeText(latitude + " " + longitude); }
function switchRulerMode() {
    rulerMode = !rulerMode;
    ruler.setEnabled(rulerMode);
}
function switchGridMode() {
    gridMode = !gridMode;
    graticule.setEnabled(gridMode);
}
function clearRuler() { ruler.clear(); }
</script>

<style>
#mapControlPanel {
    position: absolute;
    width: 60%;
    bottom: 16px;
    left: 16px;
    gap: 8px;
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
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: medium;
    border-bottom: 2px solid currentColor;
    font-size: medium;
    border-radius: 0px;
}

.scale-tick {
    position: absolute;
    border-left: 2px solid currentColor;
    height: 6px;
    bottom: 0px;
}

#ruler-label {
    width: 96px;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: medium;
}

</style>

<img id=cross-aim hidden={!crossMode} src={crossImg} />

<div id="mapControlPanel">
    <!-- Compass -->
    <button class="btn btn-lg btn-circle"  on:click={resetCompas}>
        <div style="transform:rotate({heading}deg);">{@html compasIcon}</div>
    </button>

    <!-- Coordinates -->
    <div class="join" >
        <button class="btn btn-sm join-item px-1 ml-2" on:click={switchCrossMode}>
            {@html crossMode === true ? crossIcon : cursorIcon}
        </button>
        <button class="btn btn-sm btn-wide join-item" on:click={coordsToClipboard}>
            {latitude + ", " + longitude}
        </button>
    </div>

    <!-- Map scale -->
    <div class="join bg-base-200">
        <button class="btn btn-sm px-1 join-item"
            on:mousedown={() => zoomOutPressed = true} on:mouseup={() => zoomOutPressed = false} on:mouseleave={() => zoomOutPressed = false}>
            {@html minusIcon}
        </button>
        <div id="scale" class="" bind:clientWidth={scaleWidth}>
            {metersRounded > 1000 ? (metersRounded / 1000 + " km") : (metersRounded + " m")}
            <div class="scale-tick" style ="left: 0%"></div>
            <div class="scale-tick" style ="left: {metersRounded / metersInWidth * 100}%"></div>
        </div>
        <button class="btn btn-sm px-1 join-item"
            on:mousedown={() => zoomInPressed = true} on:mouseup={() => zoomInPressed = false} on:mouseleave={() => zoomInPressed = false}>
            {@html plusIcon}
        </button>
    </div>

    <!-- Ruler Tool -->
    <div class="join bg-base-200">
        <button class={"btn btn-sm px-2 " + (rulerMode ? "btn-accent " : "") + (rulerLength > 0 ? "join-item" : "")} 
            on:click={switchRulerMode}>
            {@html rulerIcon }
        </button>
        {#if rulerLength > 0}
        <div id="ruler-label" class="">
            {rulerLength > 1000 ? ((Math.round(rulerLength / 100) / 10).toString() + " km") : (rulerLength + " m")}
        </div>
        <button class="btn btn-sm px-1 join-item" on:click={clearRuler}>
            {@html closeIcon}
        </button>
        {/if}
    </div>

    <!-- Grid Tool -->
    <button class={"btn btn-sm px-1 " + (gridMode ? "btn-accent" : "")} on:click={switchGridMode}>{@html gridIcon}</button>

    <!-- Map Layers -->
    <div tabindex="0" class="dropdown dropdown-top dropdown-end">
        <label tabindex="0" class="btn btn-sm px-2">{@html layersIcon}</label>
        <ul class="dropdown-content z-[1] menu p-2 my-2 shadow bg-base-100 rounded-box">
            <MapLayersView layers={layers} /> 
        </ul>
    </div>
</div>
