<script lang="ts">
import Topbar from '$pages/topbar/Topbar.svelte';

import FlightPage from '$pages/flight/FlightPage.svelte';
import Communication from '$pages/communication/CommunicationPage.svelte'
import About from '$pages/about/AboutPage.svelte';

import { Page, currentPage } from '$stores/app';

</script>

<Topbar/>

<div id="app" class="flex items-center justify-center h-screen">
    <!-- Main modes, never suspend -->
    <FlightPage visible={$currentPage === Page.Flight}/>

    <!-- Aux modes -->
    {#if $currentPage === Page.Communication} <Communication/> {/if}
    {#if $currentPage === Page.About} <About/> {/if}
</div>
