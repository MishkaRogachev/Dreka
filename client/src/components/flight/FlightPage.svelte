<script lang="ts">
import Map from '$components/map/cesium/Map.svelte';
import Dashboard from '$components/flight/Dashboard.svelte';

export let visible: boolean = true
</script>

<Map visible={visible}/>
<Dashboard/>
<!-- TODO: move flight map decoration here -->