<script lang="ts">
import { onMount } from 'svelte';

import * as Cesium from 'cesium';

import { MapViewportCesium } from '$lib/map/cesium/viewport';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { MapRulerCesium } from '$lib/map/cesium/ruler';
import { MapGraticuleCesium } from '$lib/map/cesium/graticule';

import MapControl from '../common/MapControl.svelte';

let cesium: Cesium.Viewer;
let viewport: MapViewportCesium;
let interaction: MapInteractionCesium;
let ruler: MapRulerCesium
let graticule: MapGraticuleCesium

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

    viewport = new MapViewportCesium(cesium);
    interaction = new MapInteractionCesium(cesium);
    ruler = new MapRulerCesium(cesium, interaction);
    graticule = new MapGraticuleCesium(cesium);
});

</script>

<div id="cesiumContainer"></div>
{#if viewport && interaction && ruler && graticule}
    <MapControl viewport={viewport} interaction={interaction} ruler={ruler} graticule={graticule}/>
    <!-- TODO: LAYERS here! -->
{/if}