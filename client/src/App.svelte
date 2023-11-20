<script lang="ts">
import Topbar from '$components/topbar/Topbar.svelte';

import FlightPage from '$components/flight/FlightPage.svelte';
import FlightChecksModal from '$components/modals/checks/FlightChecksModal.svelte';

import CommunicationModal from '$components/modals/communication/CommunicationModal.svelte'
import VehiclesListModal from '$components/modals/vehicles/VehiclesListModal.svelte';
import About from '$components/modals/about/AboutModal.svelte';

import { Page, currentPage } from '$stores/app';

</script>

<Topbar/>

<!-- pages, never suspend -->
<div id="app" class="flex items-center justify-center h-screen">
    <FlightPage visible={$currentPage === Page.Flight} />
    <FlightChecksModal />
</div>

<!-- Modals -->
<CommunicationModal />
<VehiclesListModal />

<!-- TODO: <About/> -->
