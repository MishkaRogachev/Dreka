<script lang="ts">
import { createEventDispatcher } from "svelte";

import Button from '@smui/button';
import VehicleSelector from "$pages/topbar/VehicleSelector.svelte";

import burgerIcon from "$assets/svg/burger.svg?raw";

const dispatch = createEventDispatcher()

function openDrawer() { dispatch('openDrawer', {}); }

</script>

<style>
#topBar {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
}
</style>

<div id="topBar">
    <!-- Left -->
    <Button on:click={openDrawer}>{@html burgerIcon}</Button>
    <!-- Middle -->
    <!-- Right -->
    <VehicleSelector/>
</div>