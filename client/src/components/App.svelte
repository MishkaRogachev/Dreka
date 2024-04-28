<script lang="ts">
import { onMount } from 'svelte';

import { ClientSideEvents, EventsService } from "$services/events";
import { activeDialog } from '$stores/app';

import Topbar from '$components/topbar/Topbar.svelte';

import MapCesium from '$components/map/cesium/MapCesium.svelte';
import AerialVehicleDashboard from '$components/dashboard/AerialVehicleDashboard.svelte';

import SystemsModal from '$components/modals/systems/SystemsModal.svelte';
import CommunicationModal from '$components/modals/communication/CommunicationModal.svelte'
import VehiclesListModal from '$components/modals/vehicles/VehiclesListModal.svelte';
import AboutModal from '$components/modals/about/AboutModal.svelte';
import NoServerConnection from '$components/modals/NoServerConnection.svelte';

let isServerOnline: boolean = false;

onMount(() => {
    EventsService.subscribe(ClientSideEvents.WsConnectionOpened, () => { isServerOnline = true; });
    EventsService.subscribe(ClientSideEvents.WsConnectionClosed, () => { isServerOnline = false; });

    EventsService.init();
});

</script>

<Topbar/>

<!-- STATIC -->
<MapCesium visible={true} />
<AerialVehicleDashboard />
<SystemsModal />

<!-- DIALOGS -->
<svelte:component this={$activeDialog}/>

<!-- MODALS -->
<CommunicationModal />
<VehiclesListModal />
<AboutModal />
{#if !isServerOnline}
    <NoServerConnection />
{/if}
