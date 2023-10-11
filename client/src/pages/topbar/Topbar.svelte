<script lang="ts">
import { createEventDispatcher } from "svelte";

import Button from '@smui/button';
import SvgIcon from '$components/controls/SvgIcon.svelte';
import VehicleSelector from "$pages/topbar/VehicleSelector.svelte";
import burgerIcon from "$assets/svg/burger.svg";

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
    <Button on:click={openDrawer}><SvgIcon src={burgerIcon}/></Button>
    <!-- Middle -->
    <!-- Right -->
    <VehicleSelector/>
</div>