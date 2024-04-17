
<script lang="ts">

import { userPreferences } from "$stores/preferences";
import type { MapLayers, ImageryLayer } from "$lib/interfaces/map";

import upIcon from "$assets/svg/up.svg?raw"
import downIcon from "$assets/svg/down.svg?raw"

export let layers: MapLayers

let imageryLayers = layers.imageryLayers();

function saveLayers() {
    $userPreferences.set("map/imagery_layers", JSON.stringify(layers.imageryLayers()));
}

function switchLayerVisibility(imageryLayer: ImageryLayer) {
    imageryLayer.visibility = !imageryLayer.visibility;
    layers.updateImageryLayer(imageryLayer);
    imageryLayers = layers.imageryLayers();
}

function switchLayerOpacity(imageryLayer: ImageryLayer) {
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

function raiseLayer(imageryLayer: ImageryLayer) {
    layers.raiseImageryLayer(imageryLayer);
    imageryLayers = layers.imageryLayers();
}

function lowerLayer(imageryLayer: ImageryLayer) {
    layers.lowerImageryLayer(imageryLayer);
    imageryLayers = layers.imageryLayers();
}
</script>

<table class="table" style="width: 600px">
    <thead>
        <tr>
            <td>Visible</td>
            <td>Layer</td>
            <td>Move</td>
            <td>Opacity</td>
        </tr>
    </thead>
    <tbody>
        {#each imageryLayers as imageryLayer}
        <tr>
            <td><input type="checkbox" class="checkbox" checked={imageryLayer.visibility} on:click={ () => { switchLayerVisibility(imageryLayer) }} /></td>
            <td>{imageryLayer.name}</td>
            <td class="join">
                <button class="btn btn-sm btn-ghost px-1 join-item" disabled={imageryLayer.index > 1} on:click={ () => { raiseLayer(imageryLayer) }}>
                    {@html downIcon}</button>
                <button class="btn btn-sm btn-ghost px-1 join-item" disabled={imageryLayer.index < imageryLayers.length - 2} on:click={ () => { lowerLayer(imageryLayer) }}>
                    {@html upIcon}</button>
            </td>
            <td><button class="btn btn-sm btn-ghost px-1" on:click={ () => { switchLayerOpacity(imageryLayer) }}>
                {Math.round(imageryLayer.opacity * 100) + "%"}
            </button></td> 
        </tr>
        {/each}
    </tbody>
</table>
<!-- / TODO: Map layers order like here https://sandcastle.cesium.com/index.html?src=Imagery%2520Layers%2520Manipulation.html -->
