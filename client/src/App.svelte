<script lang="ts">
import { onMount } from 'svelte';

import { EventsService } from "$services/events";

import Topbar from '$components/topbar/Topbar.svelte';

import FlightPage from '$components/flight/FlightPage.svelte';
import SystemsModal from '$components/modals/systems/SystemsModal.svelte';

import CommunicationModal from '$components/modals/communication/CommunicationModal.svelte'
import VehiclesListModal from '$components/modals/vehicles/VehiclesListModal.svelte';
import AboutModal from '$components/modals/about/AboutModal.svelte';

import { Page, currentPage } from '$stores/app';

onMount(() => {
    EventsService.init();
});

</script>

<Topbar/>

<!-- pages, never suspend -->
<div id="app" class="flex items-center justify-center h-screen">
    <FlightPage visible={$currentPage === Page.Flight} />
    <SystemsModal />
</div>

<!-- Modals -->
<CommunicationModal />
<VehiclesListModal />
<AboutModal />
