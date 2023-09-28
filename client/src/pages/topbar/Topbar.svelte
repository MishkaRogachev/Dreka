<script lang="ts">
import Led from '$components/controls/Led.svelte';
import Combobox from '$components/controls/Combobox.svelte';

import VehicleSelector from '$pages/topbar/VehicleSelector.svelte';

import { isServerOnline } from '$stores/app';

export let pages: Array<string>
export let currentPage: string = pages[0]
</script>

<style>
#topbar {
    width: 100%;
    height: 25px;
    border-radius: 0px;
    display: flex;
    flex-direction: row;
    overflow: hidden;
}

.column {
    width: 33.3%;
    height: 100%;
    display: flex;
}

</style>

<div id="topbar" class="frame">
    <div class="column" style="justify-content: flex-start√">
        <Led state={$isServerOnline ? "on" : "off"}/>
    </div>
    <div class="column" style="justify-content: center">
        <Combobox items={pages} selectedIndex={pages.indexOf(currentPage)}
            on:selected={event => {currentPage = pages[event.detail.index]}}/>
    </div>
    <div class="column" style="justify-content: flex-end" >
    <VehicleSelector/>
    </div>
</div>
