<script lang="ts">
import Led from '$components/controls/Led.svelte';
import Combobox from '$components/controls/Combobox.svelte';

import { isServerOnline } from '../../stores/app-store';

export let pages: Array<string>
export let currentPage: string = pages[0]
</script>

<style>
#topbar {
    width: 100%;
    height: 25px;
    border-radius: 0px;
    text-align: center;
    align-items: center;
}
</style>

<div id="topbar" class="frame">
    <Led style="float: left" state={$isServerOnline ? "on" : "off"}/>
    <Combobox items={pages} selectedIndex={pages.indexOf(currentPage)}
    on:selected={event => {currentPage = pages[event.detail.index]}}/>
</div>
