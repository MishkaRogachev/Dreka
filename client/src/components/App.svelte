<script lang="ts">
import { onMount } from 'svelte';

import { EventsService } from "$services/events";

import Topbar from '$components/topbar/Topbar.svelte';

import MapFacade from '$components/map/cesium/MapFacade.svelte';
import AerialVehicleDashboard from '$components/dashboard/AerialVehicleDashboard.svelte';
import SystemsModal from '$components/modals/systems/SystemsModal.svelte';

import CommunicationModal from '$components/modals/communication/CommunicationModal.svelte'
import VehiclesListModal from '$components/modals/vehicles/VehiclesListModal.svelte';
import AboutModal from '$components/modals/about/AboutModal.svelte';

onMount(() => {
    console.log("App mounted, initializing events service...");
    EventsService.init();
});

</script>

<Topbar/>

<!-- pages, never suspend -->
<div id="app" class="flex items-center justify-center grow">
    <MapFacade visible={true} />
    <AerialVehicleDashboard />
    <SystemsModal />
</div>

<!-- Modals -->
<CommunicationModal />
<VehiclesListModal />
<AboutModal />
