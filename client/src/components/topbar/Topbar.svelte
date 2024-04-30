<script lang="ts">
import { selectedVehicle } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";

import Menu from "./Menu.svelte";
import VehicleSelector from "./VehicleSelector.svelte";
import VehicleModeSelector from "$components/topbar/VehicleModeSelector.svelte";
import VehicleWaypointSelector from "$components/topbar/VehicleWaypointSelector.svelte";
import MissionControl from "$components/topbar/MissionControl.svelte";

import VehicleStateIndicator from "./VehicleStateIndicator.svelte";
import VehicleSensors from "$components/topbar/VehicleSensors.svelte";
import VehicleGps from "$components/topbar/VehicleGps.svelte";
import VehicleBattery from "./VehicleBattery.svelte";
import VehicleRadioCtrl from "$components/topbar/VehicleRadioCtrl.svelte";
import VehicleArmIndicator from "$components/topbar/VehicleArmIndicator.svelte";

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
        <VehicleStateIndicator vehicleStatus={$selectedVehicle?.status}/>
        <VehicleSensors system={$selectedVehicleTelemetry.system}/>
        <VehicleGps sns={$selectedVehicleTelemetry.rawSns}/>
        <VehicleBattery system={$selectedVehicleTelemetry.system}/>
        <VehicleRadioCtrl system={$selectedVehicleTelemetry.system}/>
        <VehicleArmIndicator/>
    </div>
</div>
