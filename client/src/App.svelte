<script lang="ts">
import Topbar from '$components/topbar/Topbar.svelte';

import FlightPage from '$components/flight/FlightPage.svelte';
import Communication from '$components/communication/CommunicationPage.svelte'
import About from '$components/about/AboutPage.svelte';

import { Page, currentPage } from '$stores/app';

</script>

<Topbar/>

<div id="app" class="flex items-center justify-center h-screen">
    <!-- Main modes, never suspend -->
    <FlightPage visible={$currentPage === Page.Flight}/>
</div>

<!-- Aux modes -->
<Communication/>
<!-- <About/> -->

