<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import MapLayersView from './MapLayersView.svelte';

import { degreesToDmsString, roundTo125 } from "$lib/common/formats";
import type { MapViewport, MapInteraction, MapRuler, MapGraticule, MapLayers } from "$lib/interfaces/map";

import { userPreferences } from '$stores/preferences';
import { i18n } from '$stores/i18n';

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
    // Update UI every 250ms
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

        $userPreferences.set("map/viewport", JSON.stringify(viewport.save()));
        $userPreferences.set("map/imagery_layers", JSON.stringify(layers.imageryLayers()));
    }, 250);
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

<img id=cross-aim hidden={!crossMode} src={crossImg} />

<div id="mapControlPanel">
    <!-- Compass -->
    <div class="tooltip" data-tip={ $i18n.t("To North") }>
        <button class="btn btn-lg btn-circle" on:click={resetCompas}>
            <div style="transform:rotate({heading}deg);">{@html compasIcon}</div>
        </button>
    </div>

    <!-- Coordinates -->
    <div class="join btn-sm p-0" >
        <div class="tooltip" data-tip={ crossMode ? $i18n.t("Cross coordinates") : $i18n.t("Mouse coordinates") }>
            <button class="btn btn-sm join-item px-1 ml-2" on:click={switchCrossMode}>
                {@html (crossMode ? crossIcon : cursorIcon)}
            </button>
        </div>
        <div class="tooltip" data-tip={ $i18n.t("Copy to clipboard") }>
        <button class="btn btn-sm join-item font-mono flex-nowrap w-72" on:click={coordsToClipboard}>
            <a>{ latitude + " " + longitude }</a>
        </button>
        </div>
    </div>

    <!-- Map scale -->
    <div class="join btn-sm p-0">
        <div class="tooltip" data-tip={ $i18n.t("Zoom out") }>
            <button class="btn btn-sm px-1 join-item"
                on:mousedown={() => zoomOutPressed = true} on:mouseup={() => zoomOutPressed = false} on:mouseleave={() => zoomOutPressed = false}>
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
                on:mousedown={() => zoomInPressed = true} on:mouseup={() => zoomInPressed = false} on:mouseleave={() => zoomInPressed = false}>
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
    <div class="tooltip" data-tip={ gridMode ? $i18n.t("Disable grid") : $i18n.t("Enable grid") }>
        <button class={"btn btn-sm px-2 " + (gridMode ? "btn-accent" : "")} on:click={switchGridMode}>{@html gridIcon}</button>
    </div>

    <!-- Map Layers -->
    <div class="tooltip" data-tip={ $i18n.t("Map layers") }>
        <div tabindex="0" class="dropdown dropdown-top dropdown-end">
            <label tabindex="0" class="btn btn-sm px-2">{@html layersIcon}</label>
            <ul class="dropdown-content z-[1] menu p-2 my-2 shadow bg-base-100 rounded-box">
                <MapLayersView layers={layers} />
            </ul>
        </div>
    </div>
</div>
