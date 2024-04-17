<script lang="ts">
import { dashboardVisible } from "$stores/app";
import { i18n } from "$stores/i18n";
import { selectedVehicle } from "$stores/vehicles";

import Menu from "./Menu.svelte";
import VehicleSelector from "./VehicleSelector.svelte";
import VehicleModeSelector from "$components/topbar/VehicleModeSelector.svelte";
import VehicleWaypointSelector from "$components/topbar/VehicleWaypointSelector.svelte";
import MissionControl from "$components/topbar/MissionControl.svelte";
import VehicleStatus from "./VehicleStatus.svelte";

</script>

<div class="navbar bg-base-300 btn-sm px-1 font-bold flex items-center gap-x-1">
    <div class="navbar-start">
        <Menu/>
        <VehicleSelector/>
        {#if !!$selectedVehicle}
            <VehicleModeSelector vehicle={ $selectedVehicle }/>
            <VehicleWaypointSelector vehicle={ $selectedVehicle }/>
            <MissionControl vehicle={ $selectedVehicle }/>
        {/if}
    </div>

    <div class="navbar-end">
        <VehicleStatus/>
    </div>
</div>