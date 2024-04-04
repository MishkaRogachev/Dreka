<script lang="ts">
import { onMount } from 'svelte';

import Menu from "./Menu.svelte";
import VehicleSelector from "./VehicleSelector.svelte";
import VehicleSummary from "./VehicleSummary.svelte";

$: dateTime = new Date()

onMount(() => {
    const interval = setInterval(() => {
        dateTime = new Date();
    }, 1000);

    return () => {
        clearInterval(interval);
    };
});
</script>

<div class="navbar bg-base-300 btn-sm">
    <div class="navbar-start">
        <Menu/>
    </div>

    <div class="navbar-end">
        <VehicleSelector/>
        <VehicleSummary/>
    </div>
</div>