<script lang="ts">
import Button from "$components/controls/Button.svelte";

import { degreesToDmsString, roundTo125 } from "$lib/common/formats";
import type { MapViewport, MapInteraction, MapRuler, MapGraticule } from "$lib/interfaces/map";

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

const scaleFactor: number = 10

const btnStyle = "display: inline; margin: 0px 2px; float: left;"

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

setInterval(() => {
    heading = viewport.heading();
    pixelScale = viewport.pixelScale();
    metersInWidth = pixelScale * scaleWidth;
    metersRounded = roundTo125(metersInWidth);

    let geodetic = crossMode ? viewport.screenXYToGeodetic({ x: viewport.viewportWidth() / 2, 
                                                            y: viewport.viewportHeight() / 2 }) :
                                viewport.screenXYToGeodetic(interaction.mouseCoordinates());
    latitude = degreesToDmsString(geodetic.latitude, false);
    longitude = degreesToDmsString(geodetic.longitude, true);

    rulerLength = Math.round(ruler.distance())

    if (zoomInPressed) {
        viewport.zoomIn(pixelScale * scaleFactor);
    }

    if (zoomOutPressed) {
        viewport.zoomOut(pixelScale * scaleFactor);
    }
}, 50)

function resetCompas() { viewport.lookTo(0, -90, 2); }
function coordsToClipboard() { navigator.clipboard.writeText(latitude + " " + longitude); }
function switchRulerMode() {
    rulerMode = !rulerMode
    ruler.setEnabled(rulerMode)
}

function switchGridMode() {
    gridMode = !gridMode
    graticule.setEnabled(gridMode)
}

function clearRuler() {
    ruler.clear()
}
</script>

<style>
#mapControlPanel {
    position: absolute;
    bottom: 10px;
    left: 10px;
    height: 24px;
    background: transparent;
    display: inline;
}
button {
    display: inline;
    margin: 0px 2px;
    float: left;
}
#compas-button {
    padding: 0px 0px;
    width: 32px;
    height: 32px;
    border-radius: 16px;
    margin: -5px 2px;
}
#cross-aim {
    position: absolute;
    top: 50%;
    left: 50%;
}
#scale {
    width: 128px;
    float: left;
    border-bottom: 2px solid white;
    margin-left: -1px;
    margin-right: -1px;
}
#ruler-label {
    width: 96px;
    line-height: 24px;
    float: left;
    margin-left: -1px;
    margin-right: -1px;
}
.scale-tick {
    position: absolute;
    border-left: 2px solid white;
    height: 6px;
    bottom: 0px;
}
</style>

<img id=cross-aim src={crossIcon} width=24px height=24px alt="Cross" hidden={!crossMode} />

<div id="mapControlPanel">
    <button id="compas-button" on:click={resetCompas}>
        <img src={compasIcon} alt="Compas" style="transform:rotate({heading}deg);" />
    </button>
    <Button style={btnStyle} right_cropped={true} icon={crossMode === true ? crossIcon : cursorIcon}
        on:click={() => { crossMode = !crossMode }}/>
    <Button style={btnStyle + "width: 216px; margin-left: -1px;"} left_cropped={true} text={latitude + ", " + longitude}
        on:click={coordsToClipboard}/>
    <Button style={btnStyle} right_cropped={true} icon={minusIcon}
        on:mousedown="{() => zoomOutPressed = true}"
        on:mouseup="{() => zoomOutPressed = false}"
        on:mouseleave="{() => zoomOutPressed = false}"/>
    <div id="scale" class="pane noselect left-cropped right-cropped" bind:clientWidth={scaleWidth}>
        {metersRounded > 1000 ? (metersRounded / 1000 + " km") : (metersRounded + " m")}
        <div class="scale-tick" style ="left: 0%"></div>
        <div class="scale-tick" style ="left: {metersRounded / metersInWidth * 100}%"></div>
    </div>
    <Button style={btnStyle} left_cropped={true} icon={plusIcon}
        on:mousedown="{() => zoomInPressed = true}"
        on:mouseup="{() => zoomInPressed = false}"
        on:mouseleave="{() => zoomInPressed = false}"/>
    <Button style={btnStyle} right_cropped={rulerLength > 0} selected={rulerMode} icon={rulerIcon} on:click={switchRulerMode}/>
    {#if rulerLength > 0}
        <div id="ruler-label" class="pane noselect left-cropped right-cropped">
            {rulerLength > 1000 ? ((Math.round(rulerLength / 100) / 10).toString() + " km") : (rulerLength + " m")}
        </div>
        <Button style={btnStyle} left_cropped={true} icon={closeIcon} on:click={clearRuler}/>
    {/if}
    <Button style={btnStyle} selected={gridMode} icon={gridIcon} on:click={switchGridMode}/>
</div>
