<script lang="ts">
import Topbar from '$pages/topbar/Topbar.svelte';

import FlightPage from '$pages/flight/FlightPage.svelte';
import Communication from '$pages/communication/CommunicationPage.svelte'
import About from '$pages/about/AboutPage.svelte';

import { Page, currentPage, Modal, currentModal } from '$stores/app';

</script>

<Topbar/>

<div id="app" class="flex items-center justify-center h-screen">
    <!-- Main modes, never suspend -->
    <FlightPage visible={$currentPage === Page.Flight}/>
</div>

<!-- Aux modes -->
{#if $currentModal === Modal.Communication} <Communication/> {/if}
{#if $currentModal === Modal.About} <About/> {/if}

