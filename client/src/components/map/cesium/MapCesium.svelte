<script lang="ts">
import { onMount } from 'svelte';
import * as Cesium from 'cesium';

let viewer: Cesium.Viewer;
onMount(async () => {
    viewer = new Cesium.Viewer(
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
    viewer.resolutionScale = window.devicePixelRatio;
});

</script>

<div id="cesiumContainer"></div>