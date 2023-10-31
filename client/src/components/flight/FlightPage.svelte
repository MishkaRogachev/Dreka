<script lang="ts">
import MapCesium from '$components/map/cesium/MapCesium.svelte';
import Dashboard from '$components/flight/Dashboard.svelte';

export let visible: boolean = true
</script>

<MapCesium visible={visible}/>
<Dashboard/>
