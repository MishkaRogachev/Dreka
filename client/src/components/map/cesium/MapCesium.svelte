<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import * as Cesium from 'cesium';

import { preferences } from "$lib/preferences";
import { MapViewportCesium } from '$lib/map/cesium/viewport';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { MapRulerCesium } from '$lib/map/cesium/ruler';
import { MapGraticuleCesium } from '$lib/map/cesium/graticule';
import { MapLayersCesium } from '$lib/map/cesium/layers';

import MapControl from '../common/MapControl.svelte';

export let visible: boolean = true

let cesium: Cesium.Viewer;
let viewport: MapViewportCesium;
let interaction: MapInteractionCesium;
let ruler: MapRulerCesium;
let graticule: MapGraticuleCesium;
let layers: MapLayersCesium;

let ready: boolean = false;
let interval: any;

onMount(async () => {
    cesium = new Cesium.Viewer(
        'cesiumContainer', {
            orderIndependentTranslucency: false,
            timeline: false,
            geocoder: false,
            selectionIndicator: false,
            infoBox: false,
            scene3DOnly: true,
            shouldAnimate: true,
            baseLayerPicker: false,
        });
    cesium.resolutionScale = window.devicePixelRatio;

    // TODO: terrain layers
    cesium.terrainProvider = await Cesium.createWorldTerrainAsync({
        requestVertexNormals: true,
        requestWaterMask: true
    });

    viewport = new MapViewportCesium(cesium);
    const viewportSettings = preferences.read("user/map/viewport")
    if (!!viewportSettings) {
        viewport.restore(JSON.parse(viewportSettings));
    }

    interaction = new MapInteractionCesium(cesium);
    ruler = new MapRulerCesium(cesium, interaction);
    graticule = new MapGraticuleCesium(cesium);
    
    layers = new MapLayersCesium(cesium);
    const layerSettings = preferences.read("user/map/imagery_layers");
    if (!!layerSettings) {
        layers.addImageryLayers(JSON.parse(layerSettings));
    } else {
        layers.resetImageryLayers();
    }

    ready = true;

    // Save map settings every second
    interval = setInterval(() => {
        preferences.write("user/map/viewport", JSON.stringify(viewport.save()));
        preferences.write("user/map/imagery_layers", JSON.stringify(layers.imageryLayers()));
    }, 1000);
});

onDestroy(async () => { clearInterval(interval); ready = false; });

</script>

<div id="cesiumContainer" style={ready && visible ? "" : "display: none"}>
{#if ready && visible}
    <MapControl viewport={viewport} interaction={interaction} ruler={ruler} graticule={graticule} layers={layers}/>
{/if}
</div>
