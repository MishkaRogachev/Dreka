<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import { MapFacadeCesium } from '$lib/map/cesium/facade';
import { userPreferences } from '$stores/preferences';
import { mainMap } from '$stores/app';

import MapControl from '$components/map/common/MapControl.svelte';
import MapMenu from '$components/map/common/MapMenu.svelte';
import MapDrawVehicles from '$components/map/common/MapDrawVehicles.svelte';
import MapDrawMissionRoutes from '$components/map/common/MapDrawMissionRoutes.svelte';

export let visible: boolean = true

let map: MapFacadeCesium;

let ready: boolean = false;
let interval: any;

onMount(async () => {
    map = new MapFacadeCesium('cesiumContainer');
    await map.initTerrain();

    const viewportSettings = $userPreferences.get("map/viewport")
    if (!!viewportSettings) {
        map.viewport.restore(JSON.parse(viewportSettings));
    }

    const layerSettings = $userPreferences.get("map/imagery_layers");
    if (!!layerSettings) {
        map.layers.addImageryLayers(JSON.parse(layerSettings));
    } else {
        map.layers.resetImageryLayers();
    }

    // TODO: await cesium loaded
    ready = true;
    mainMap.set(map);
});

onDestroy(async () => { clearInterval(interval); ready = false; });

</script>

<span class="loading loading-ring loading-lg" style={visible && !ready ? "" : "display: none"}></span>

<div id="cesiumContainer" class="absolute" style={ready && visible ? "" : "display: none"}>
{#if ready && visible}
    <MapMenu interaction={map.interaction} viewport={map.viewport}/>
    <MapDrawVehicles mapVehicles={map.vehicles} viewport={map.viewport}/>
    <MapDrawMissionRoutes map={map}/>
{/if}
</div>
{#if ready && visible}
    <MapControl map={map}/>
{/if}
