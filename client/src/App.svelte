<script lang="ts">
import Topbar from '$components/topbar/Topbar.svelte';

import FlightPage from '$components/flight/FlightPage.svelte';
import CommunicationModal from '$components/modals/communication/CommunicationModal.svelte'
import VehiclesModal from '$components/modals/vehicles/VehiclesModal.svelte';
import About from '$components/modals/about/AboutModal.svelte';

import { Page, currentPage } from '$stores/app';

</script>

<Topbar/>

<div id="app" class="flex items-center justify-center h-screen">
    <!-- Main modes, never suspend -->
    <FlightPage visible={$currentPage === Page.Flight}/>
</div>

<!-- Aux modes -->
<CommunicationModal/>
<VehiclesModal/>
<!-- <About/> -->
