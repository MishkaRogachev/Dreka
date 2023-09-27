<script lang="ts">
import Topbar from '$pages/topbar/Topbar.svelte';
import Flight from '$pages/flight/Flight.svelte';
import Settings from '$pages/settings/Settings.svelte';

enum Pages {
    Flight = "Flight",
    Settings = "Settings"
}

const pages = [Pages.Flight, Pages.Settings];
let currentPage: Pages = Pages.Flight;
</script>

<div id="app" style="background-color:black">
    <Topbar pages={pages} bind:currentPage = {currentPage} />
    <Flight visible={currentPage == Pages.Flight}/>
    <Settings visible={currentPage == Pages.Settings}/>
</div>
