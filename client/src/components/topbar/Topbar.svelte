<script lang="ts">
import { selectedVehicle } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";

import Menu from "./Menu.svelte";
import VehicleSelector from "./VehicleSelector.svelte";
import VehicleModeSelector from "$components/topbar/VehicleModeSelector.svelte";
import VehicleWaypointSelector from "$components/topbar/VehicleWaypointSelector.svelte";
import MissionControl from "$components/topbar/MissionControl.svelte";

import VehicleBattery from "./VehicleBattery.svelte";
import VehicleGps from "$components/topbar/VehicleGps.svelte";
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
        <VehicleBattery system={$selectedVehicleTelemetry.system}/>
        <VehicleGps sns={$selectedVehicleTelemetry.rawSns}/>
        <VehicleStatus/>
    </div>
</div>