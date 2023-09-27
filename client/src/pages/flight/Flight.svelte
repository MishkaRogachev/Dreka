<script lang="ts">
import MapCesium from '$components/map/cesium/MapCesium.svelte';

export let visible: boolean = true
</script>

<MapCesium visible={visible}/>
