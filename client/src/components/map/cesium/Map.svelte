<script lang="ts">
import { onMount, onDestroy } from 'svelte';
import * as Cesium from 'cesium';

import { MapViewportCesium } from '$lib/map/cesium/viewport';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { MapRulerCesium } from '$lib/map/cesium/ruler';
import { MapGraticuleCesium } from '$lib/map/cesium/graticule';
import { MapLayersCesium } from '$lib/map/cesium/layers';

import { userPreferences } from '$stores/preferences';

import MapControl from '../common/MapControl.svelte';
import MapMenu from '$components/map/common/MapMenu.svelte';

import MapVehicles from '$components/map/cesium/MapVehicles.svelte';
import MapMissionRoutes from '$components/map/cesium/MapMissionRoutes.svelte';

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
    const viewportSettings = $userPreferences.get("map/viewport")
    if (!!viewportSettings) {
        viewport.restore(JSON.parse(viewportSettings));
    }

    interaction = new MapInteractionCesium(cesium);
    ruler = new MapRulerCesium(cesium, interaction);
    graticule = new MapGraticuleCesium(cesium);
    
    layers = new MapLayersCesium(cesium);
    const layerSettings = $userPreferences.get("map/imagery_layers");
    if (!!layerSettings) {
        layers.addImageryLayers(JSON.parse(layerSettings));
    } else {
        layers.resetImageryLayers();
    }

    // TODO: await cesium loaded
    ready = true;
});

onDestroy(async () => { clearInterval(interval); ready = false; });

</script>

<span class="loading loading-ring loading-lg" style={visible && !ready ? "" : "display: none"}></span>

<div id="cesiumContainer" class="absolute" style={ready && visible ? "" : "display: none"}>
{#if ready && visible}
    <MapMenu interaction={interaction} viewport={viewport}/>
    <MapVehicles cesium={cesium} interaction={interaction}/>
    <MapMissionRoutes cesium={cesium} viewport={viewport} interaction={interaction}/>
{/if}
</div>
{#if ready && visible}
    <MapControl viewport={viewport} interaction={interaction} ruler={ruler} graticule={graticule} layers={layers}/>
{/if}

