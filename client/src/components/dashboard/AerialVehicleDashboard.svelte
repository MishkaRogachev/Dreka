<script lang="ts">
import Ai from "$components/dashboard/indicators/AI.svelte";
import Hsi from "$components/dashboard/indicators/HSI.svelte";
import Parameter from "$components/dashboard/indicators/Parameter.svelte";
import VehicleTypeIcon from "$components/common/VehicleTypeIcon.svelte";

import { dashboardVisible, mainMap } from '$stores/app';
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
$: mapVehicle = $mainMap?.vehicles.vehicle($selectedVehicle?.description.id || "")

let isTracking: boolean = false;

function trackButtonShortpress() {
    if (!mapVehicle) return;

    if (isTracking) {
        mapVehicle.setTracking(false);
        isTracking = false;
    } else {
        mapVehicle.centerOnMap();
    }
}

function trackButtonLongpress() {
    if (!mapVehicle) return;

    if (isTracking) {
        mapVehicle.setTracking(false);
        isTracking = false;
    } else {
        mapVehicle.setTracking(true);
        isTracking = true;
    }
}

function coordsToClipboard() {
    navigator.clipboard.writeText(geodeticCoordinates.join(";"));
}

</script>
<div id="dashboard" class="absolute top-10 right-2 bg-base-300 p-0 rounded-md shadow-lg">
    <!-- TITLE -->
    <div class={"flex align-center " + ($dashboardVisible ? "p-2" : "p-0") } >
        {#if $dashboardVisible}
        <div class="tooltip tooltip-left" data-tip={ $i18n.t("Center vehicle on map") }>
            <button class={ "btn btn-sm btn-circle px-1 " + (isTracking ? "btn-accent" : "btn-ghost") }
                use:longpress={{ delay: 500, repeat: false, onLongPress: trackButtonLongpress, onIdleClick: trackButtonShortpress}}>
                {@html centerIcon}</button>
        </div>
        <div class="tooltip tooltip-left grow h-full" data-tip={ $i18n.t("Vehicle coordinates, click to copy") }>
            <button class="btn btn-xs btn-ghost px-1 font-mono text-right grow h-full" on:click={coordsToClipboard}>
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

    <!-- FLIGHT DATA DISPLAY -->
        <Parameter name={ $i18n.t("GS") } tooltip={ $i18n.t("Ground (GPS) Speed") }
            value={ telemetry.flight.ground_speed}/>
        <div class="row-span-3 col-span-2 flex items-center justify-center">
            <Ai online={online}
                pitch={telemetry.flight.pitch}
                roll={telemetry.flight.roll}
            />
        </div>
        <Parameter name={ $i18n.t("ASNS") } tooltip={ $i18n.t("Satellite (GPS) Altitude") }
            value={ telemetry.navigation.position.altitude}/>
        <Parameter name={ $i18n.t("IAS") } tooltip={ $i18n.t("Indicated Air Speed") }
            value={ telemetry.flight.indicated_airspeed}/>
        <Parameter name={ $i18n.t("AMSL") } tooltip={ $i18n.t("Altitude Above Mean Sea Level") }
            value={ telemetry.flight.altitude_amsl}/>
        <a href={null} class="text-sm">{ $i18n.t("m/s") }</a>
        <a href={null} class="text-sm">{ $i18n.t("m") }</a>

    <!-- NAVIGATION DATA DISPLAY -->
        <Parameter name={ $i18n.t("HDG") } tooltip={ $i18n.t("Heading") }
            value={ formatHeading(telemetry.flight.yaw) }/>
        <div class="row-span-3 col-span-2 flex items-center justify-center">
            <Hsi
                heading={telemetry.flight.yaw}
                course={telemetry.rawSns.course}
                courseEnabled={telemetry.rawSns.ground_speed > 1}
            />
            <div id="vehicleMark" class="absolute flex items-center justify-center scale-150">
                <VehicleTypeIcon 
                    vehicleType={$selectedVehicle?.description.vehicle_type} 
                    color={$selectedVehicle?.description.color || ""}/>
            </div>
        </div>
        <Parameter name={ $i18n.t("WPT") } tooltip={ $i18n.t("Distance to next waypoint") }
            value={ telemetry.navigation.wp_distance}/>
        <Parameter name={ $i18n.t("CRS") } tooltip={ $i18n.t("Course (GPS)") }
            value={ formatHeading(telemetry.rawSns.course) }/>
        <Parameter name={ $i18n.t("HOME") } tooltip={ $i18n.t("Distance to home point") }
            value={ $mainMap ? $mainMap.calcDistance(telemetry.navigation.position, telemetry.navigation.home_position) : 0}/> <!-- TODO: home distance -->
        <a href={null} class="text-sm">&deg</a>
        <a href={null} class="text-sm">{ $i18n.t("m") }</a>
    </div>
    {/if}
</div>
