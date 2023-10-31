<script lang="ts">
import MapCesium from '$components/map/cesium/MapCesium.svelte';
import Dashboard from '$components/dashboard/Dashboard.svelte';

export let visible: boolean = true
</script>

<MapCesium visible={visible}/>
<Dashboard visible={visible}/>
