<script lang="ts">
import Topbar from '$pages/topbar/Topbar.svelte';

import FlightPage from '$pages/flight/FlightPage.svelte';
import Communication from '$pages/communication/CommunicationPage.svelte'
import About from '$pages/about/AboutPage.svelte';

import { Pages, currentPage } from '$stores/app';

</script>

<div id="app" style="background-color:black">
    <Topbar/>
    <!-- Main modes, never suspend -->
    <FlightPage visible={$currentPage === Pages.Flight}/>

    <!-- Aux modes -->
    {#if $currentPage === Pages.Communication} <Communication/> {/if}
    {#if $currentPage === Pages.About} <About/> {/if}
</div>
