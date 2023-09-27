<script lang="ts">
import Combobox from '$components/controls/Combobox.svelte';
import Flight from '$pages/flight/Flight.svelte';
import Settings from '$pages/settings/Settings.svelte';

enum Pages {
    Flight = "Flight",
    Settings = "Settings"
}

const pages = [Pages.Flight, Pages.Settings]
let currentPage: Pages = Pages.Flight

</script>

<style>
#topbar {
    width: 100%;
    height: 25px;
    border-radius: 0px;
}
</style>

<div id="topbar" class="frame">
    <Combobox items={pages} selectedIndex={pages.indexOf(currentPage)}
    on:selected={event => {currentPage = pages[event.detail.index]}}/>
</div>

{#if currentPage == Pages.Flight}
    <Flight />
{/if}
{#if currentPage == Pages.Settings}
    <Settings />
{/if}

