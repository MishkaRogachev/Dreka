
<script lang="ts">
import type { MapLayers, ImageryLayer } from "$lib/interfaces/map";

import upIcon from "$assets/svg/up.svg"
import downIcon from "$assets/svg/down.svg"

export let layers: MapLayers

let imageryLayers = layers.imageryLayers();

async function switchLayerVisibility(imageryLayer: ImageryLayer) {
    imageryLayer.visibility = !imageryLayer.visibility;
    layers.updateImageryLayer(imageryLayer);
    imageryLayers = layers.imageryLayers();
}

async function switchLayerOpacity(imageryLayer: ImageryLayer) {
    if (imageryLayer.opacity < 0.25 || imageryLayer.opacity >= 1.0) {
        imageryLayer.opacity = 0.25;
    } else if (imageryLayer.opacity < 0.50) {
        imageryLayer.opacity = 0.50;
    } else if (imageryLayer.opacity < 0.75) {
        imageryLayer.opacity = 0.75;
    } else {
        imageryLayer.opacity = 1.0;
    }
    layers.updateImageryLayer(imageryLayer);
    imageryLayers = layers.imageryLayers();
}

async function raiseLayer(imageryLayer: ImageryLayer) {
    layers.raiseImageryLayer(imageryLayer);
    imageryLayers = layers.imageryLayers();
}

async function lowerLayer(imageryLayer: ImageryLayer) {
    layers.lowerImageryLayer(imageryLayer);
    imageryLayers = layers.imageryLayers();
}
</script>

<style>
td {
    text-align: left;
}
</style>

<div style="width: 380px; max-height: 256px">
<table class="noselect" style="width:100%">
    {#each imageryLayers as imageryLayer}
    <tr>
        <td><input type=checkbox checked={imageryLayer.visibility} on:click={ () => { switchLayerVisibility(imageryLayer) }}></td>
        <td>{imageryLayer.name}</td>
        <td><button class="hidden" on:click={ () => { lowerLayer(imageryLayer) }} disabled={ imageryLayer.index < 2 }>
            <img class="button-icon" src={downIcon} alt="Down" />
        </button></td>
        <td><button class="hidden" on:click={ () => { raiseLayer(imageryLayer) }} disabled={ imageryLayer.index > imageryLayers.length - 1 } >
            <img class="button-icon" src={upIcon} alt="Up" />
        </button></td>
        <td><button class="flat" style="min-width: 48px;" on:click={ () => { switchLayerOpacity(imageryLayer) }}>{Math.round(imageryLayer.opacity * 100) + "%"}</button></td>
    </tr>
    {/each}
</table>
</div>

<!-- / TODO: Map layers order like here https://sandcastle.cesium.com/index.html?src=Imagery%2520Layers%2520Manipulation.html -->
