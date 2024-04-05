<script lang="ts">
import Map from '$components/map/cesium/Map.svelte';
import Dashboard from '$components/flight/Dashboard.svelte';

import { dashboardVisible } from '$stores/app';

export let visible: boolean = true
</script>

<Map visible={visible}/>
{#if $dashboardVisible}
    <Dashboard/>
{/if}