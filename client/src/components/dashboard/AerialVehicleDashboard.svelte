<script lang="ts">
import Ai from "$components/dashboard/indicators/AI.svelte";
import Hsi from "$components/dashboard/indicators/HSI.svelte";
import Parameter from "$components/dashboard/indicators/Parameter.svelte";
import VehicleTypeIcon from "$components/common/VehicleTypeIcon.svelte";

import { dashboardVisible } from '$stores/app';
import { formatGeodeticCoordinates, i18n } from '$stores/i18n';
import { selectedVehicle } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";

import { longpress } from "$lib/common/longpress";
import { formatHeading } from "$lib/common/formats";

import centerIcon from "$assets/svg/center.svg?raw";
import hideIcon from "$assets/svg/hide_dashboard.svg?raw";
import showIcon from "$assets/svg/show_dashboard.svg?raw";

$: telemetry = $selectedVehicleTelemetry
$: online = $selectedVehicle?.is_online
$: geodeticCoordinates = formatGeodeticCoordinates(telemetry.navigation?.position)

function centerVehicle() {
    console.log("Centering vehicle on map");
} // TODO: center/track vehicle

function switchVehicleTracking() {
    console.log("Switching vehicle tracking");
} // TODO: center/track vehicle

function coordsToClipboard() {
    navigator.clipboard.writeText(geodeticCoordinates.join(";"));
}

</script>
<div id="dashboard" class="absolute top-10 right-2 bg-base-300 p-0 rounded-md shadow-lg">
    <!-- TITLE -->
    <div class="flex content-center">
        {#if $dashboardVisible}
        <div class="tooltip tooltip-left" data-tip={ $i18n.t("Center vehicle on map") }>
            <button class="btn btn-sm btn-circle btn-ghost px-1 h-full"
                use:longpress={{ delay: 500, repeat: false, onLongPress: switchVehicleTracking }}
                on:click={centerVehicle}
            >{@html centerIcon}</button>
        </div>
        <div class="tooltip tooltip-left grow" data-tip={ $i18n.t("Vehicle coordinates, click to copy") }>
            <button class="btn btn-xs btn-ghost px-1 font-mono text-right h-full" on:click={coordsToClipboard}>
                { geodeticCoordinates[0] } <br/> { geodeticCoordinates[1] }
            </button>
        </div>
        {/if}
        <div class="tooltip tooltip-left" data-tip={ $dashboardVisible ? $i18n.t("Hide dashboard") : $i18n.t("Show dashboard") }>
            <button class="btn btn-sm btn-circle btn-ghost" on:click={() => { $dashboardVisible = !$dashboardVisible }}>
                { @html $dashboardVisible ? hideIcon : showIcon}
            </button>
        </div>
    </div>

    {#if $dashboardVisible}
    <div class = "grid grid-cols-4 gap-2 w-50 text-center items-center justify-items-stretch p-2">
    
    <!-- <div class="col-span-4 join content-center">
        <h3 class="font-bold text-lg">{ $i18n.t("Dashboard") }</h3>
    </div> -->
    
        
    <!-- NAVIGATION
    <div class="join btn-sm p-0">
        <div class="tooltip tooltip-left grow" data-tip={ $i18n.t("Vehicle coordinates, click to copy") }>
            <button class="btn btn-xs px-1 join-item font-mono text-right h-full" on:click={coordsToClipboard}>
                { geodeticCoordinates.replace(";", "\n\r") }
            </button>
        </div>

    </div> -->
    <!-- FLIGHT DATA DISPLAY -->
        <Parameter name={ $i18n.t("GS") } tooltip={ $i18n.t("Ground (GPS) Speed") }
            value={telemetry.navigation ? telemetry.flight.ground_speed : 0}/>
        <div class="row-span-3 col-span-2 relative">
            <Ai online={online}
                pitch={telemetry.flight ? telemetry.flight.pitch : 0 }
                roll={telemetry.flight ? telemetry.flight.roll : 0 }
            />
        </div>
        <Parameter name={ $i18n.t("ASNS") } tooltip={ $i18n.t("Satellite (GPS) Altitude") }
            value={ telemetry.navigation ? telemetry.navigation.position.altitude : 0 }/>
        <Parameter name={ $i18n.t("IAS") } tooltip={ $i18n.t("Indicated Air Speed") }
            value={ telemetry.flight ? telemetry.flight.indicated_airspeed : 0 }/>
        <Parameter name={ $i18n.t("AMSL") } tooltip={ $i18n.t("Altitude Above Mean Sea Level") }
            value={ telemetry.flight ? telemetry.flight.altitude_amsl : 0 }/>
        <a href={null} class="text-sm">{ $i18n.t("m/s") }</a>
        <a href={null} class="text-sm">{ $i18n.t("m") }</a>

    <!-- NAVIGATION DATA DISPLAY -->
        <Parameter name={ $i18n.t("HDG") } tooltip={ $i18n.t("Heading") }
            value={ formatHeading(telemetry.flight ? telemetry.flight.yaw : 0) }/>
        <div class="row-span-3 col-span-2 relative">
            <Hsi
                heading={telemetry.flight ? telemetry.flight.yaw : 0}
                course={telemetry.navigation ? telemetry.rawSns.course : 0}
            />
            <div id="vehicleMark" class="absolute inset-0 flex items-center justify-center scale-150">
                <VehicleTypeIcon 
                    vehicleType={$selectedVehicle?.description.vehicle_type} 
                    color={$selectedVehicle?.description.color || ""}/>
            </div>
        </div>
        <Parameter name={ $i18n.t("WPT") } tooltip={ $i18n.t("Distance to next waypoint") }
            value={ telemetry.flight ? telemetry.navigation.wp_distance : 0 }/>
        <Parameter name={ $i18n.t("CRS") } tooltip={ $i18n.t("Course (GPS)") }
            value={ telemetry.navigation ? formatHeading(telemetry.rawSns.course) : 0 }/>
        <Parameter name={ $i18n.t("HOME") } tooltip={ $i18n.t("Distance to home point") }
            value={ 0 }/> <!-- TODO: home distance -->
        <a href={null} class="text-sm">&deg</a>
        <a href={null} class="text-sm">{ $i18n.t("m") }</a>
    </div>
    {/if}
</div>
