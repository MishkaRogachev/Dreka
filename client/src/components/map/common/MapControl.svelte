<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import Fab from '@smui/fab';
import Button, { Group } from '@smui/button';
import { Text } from '@smui/list';

import SvgIcon from '$components/controls/SvgIcon.svelte';
import OverlayButton from '$components/controls/OverlayButton.svelte';

import MapLayersView from './MapLayersView.svelte';

import { degreesToDmsString, roundTo125 } from "$lib/common/formats";
import type { MapViewport, MapInteraction, MapRuler, MapGraticule, MapLayers } from "$lib/interfaces/map";

import layersIcon from "$assets/svg/layers.svg"
import crossIcon from "$assets/svg/cross.svg";
import compasIcon from "$assets/svg/compas.svg";
import cursorIcon from "$assets/svg/cursor.svg";
import minusIcon from "$assets/svg/minus.svg";
import plusIcon from "$assets/svg/plus.svg";
import rulerIcon from "$assets/svg/ruler.svg";
import closeIcon from "$assets/svg/close.svg";
import gridIcon from "$assets/svg/grid.svg";

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
let layersView: MapLayersView

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
function openCloseMapLayers() { layersView.setOpen(!layersView.isOpened()) }
</script>

<style>
#mapControlPanel {
    position: absolute;
    width: 50%;
    bottom: 16px;
    left: 16px;
    gap: 8px;
    background: transparent;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: stretch;
}

#cross-aim {
    position: absolute;
    top: 50%;
    left: 50%;
}
#scale {
    width: 128px;
    border-bottom: 2px solid white;
    font-size: medium;
}

.scale-tick {
    position: absolute;
    border-left: 2px solid white;
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

<img id=cross-aim src={crossIcon} width=24px height=24px alt="Cross" hidden={!crossMode} />

<div id="mapControlPanel">
    <!-- Compass -->
    <Fab color="secondary" on:click={resetCompas}>
        <SvgIcon src={compasIcon} rotation={heading} size={48}/>
    </Fab>

    <!-- Coordinates -->
    <Group>
        <Button color="secondary" on:click={switchCrossMode} variant="raised">
            <SvgIcon src={crossMode === true ? crossIcon : cursorIcon}/>
        </Button>
        <Button color="secondary" on:click={coordsToClipboard} variant="raised">
            <Text>{latitude + ", " + longitude}</Text>
        </Button>
    </Group>

    <!-- Map scale -->
    <Group>
        <Button color="secondary" variant="raised"
        on:mousedown={() => zoomOutPressed = true} on:mouseup={() => zoomOutPressed = false} on:mouseleave={() => zoomOutPressed = false}>
            <SvgIcon src={minusIcon}/>
        </Button>
        <div id="scale" class="pane noselect left-cropped right-cropped" bind:clientWidth={scaleWidth}>
            {metersRounded > 1000 ? (metersRounded / 1000 + " km") : (metersRounded + " m")}
            <div class="scale-tick" style ="left: 0%"></div>
            <div class="scale-tick" style ="left: {metersRounded / metersInWidth * 100}%"></div>
        </div>
        <Button color="secondary" variant="raised"
        on:mousedown={() => zoomInPressed = true} on:mouseup={() => zoomInPressed = false} on:mouseleave={() => zoomInPressed = false}>
            <SvgIcon src={plusIcon}/>
        </Button>
    </Group>

    <!-- Ruler Tool -->
    <Group>
        <Button color={rulerMode ? "primary" : "secondary"} on:click={switchRulerMode} variant="raised">
            <SvgIcon src={rulerIcon}/>
        </Button>
        {#if rulerLength > 0}
            <div id="ruler-label" class="pane noselect left-cropped right-cropped">
                {rulerLength > 1000 ? ((Math.round(rulerLength / 100) / 10).toString() + " km") : (rulerLength + " m")}
            </div>
            <Button color="secondary" on:click={clearRuler} variant="raised">
                <SvgIcon src={closeIcon}/>
            </Button>
        {/if}
    </Group>

    <!-- Grid Tool -->
    <Button color={gridMode ? "primary" : "secondary"} on:click={switchGridMode} variant="raised">
        <SvgIcon src={gridIcon}/>
    </Button>

    <!-- Map Layers -->
    <div>
    <Button color="secondary" on:click={openCloseMapLayers} variant="raised">
        <SvgIcon src={layersIcon}/>
    </Button>
    <MapLayersView layers={layers} bind:this={layersView} />
    </div>
</div>
