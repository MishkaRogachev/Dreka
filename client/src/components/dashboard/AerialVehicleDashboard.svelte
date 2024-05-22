<script lang="ts">
import { dashboardVisible, mainMap } from '$stores/app';
import { formatGeodeticCoordinates, i18n } from '$stores/i18n';
import { selectedVehicle } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";

import { longpress } from "$lib/common/longpress";
import { formatHeading } from "$lib/common/formats";

import FD from "$components/dashboard/indicators/FD.svelte";
import HSI from "$components/dashboard/indicators/HSI.svelte";
import Parameter from "$components/dashboard/indicators/Parameter.svelte";
import Bar from '$components/dashboard/indicators/Bar.svelte';
import VehicleTypeIcon from "$components/common/VehicleTypeIcon.svelte";

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
                {@html centerIcon}
            </button>
        </div>
        <div class="tooltip tooltip-left grow h-full" data-tip={ $i18n.t("Vehicle coordinates, click to copy") }>
            <button class={"btn btn-xs btn-ghost px-1 font-mono text-right grow h-full " +
                (telemetry.navigation ? "" : "text-neutral")} on:click={coordsToClipboard}>
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
    <div class = "grid grid-cols-4 w-50 text-center items-center justify-items-center p-2">

    <!-- FLIGHT -->
        <Parameter name={ $i18n.t("GS") } tooltip={ $i18n.t("Ground (GPS) Speed, m/s") }
            value={ telemetry.flight?.ground_speed}/>
        <div class="row-span-2 col-span-2 flex items-center space-x-0.5">
            <div class="tooltip tooltip-left"
                data-tip={ $i18n.t("Throttle") + ": " + (telemetry.flight?.throttle.toFixed(2) || "-")}>
                <Bar value={ telemetry.flight?.throttle || 0} minValue={0} maxValue={100}/>
            </div>
            <div class="tooltip tooltip-left"
                data-tip={ $i18n.t("Pitch") + ": " + (telemetry.flight?.pitch.toFixed(2) || "-") + ", " +
                    $i18n.t("Roll") + ": " + (telemetry.flight?.roll.toFixed(2) || "-")}>
                <FD online={online} pitch={telemetry.flight?.pitch || 0} roll={telemetry.flight?.roll || 0}/>
            </div>
            <div class="tooltip tooltip-left"
                data-tip={ $i18n.t("Climb") + ": " + (telemetry.flight ? telemetry.flight.climb.toFixed(2) + " " + $i18n.t("m/s") : "-")}>
                <Bar value={ telemetry.flight?.climb || 0} minValue={-10} maxValue={10}
                    color={telemetry.flight && telemetry.flight.climb > 0 ? "#80deea" : "#fdd835"}/>
            </div>
        </div>
        <Parameter name={ $i18n.t("ASNS") } tooltip={ $i18n.t("Satellite (GPS) Altitude, m") }
            value={ telemetry.navigation?.position.altitude }/>
        <Parameter name={ $i18n.t("IAS") } tooltip={ $i18n.t("Indicated Air Speed, m/s") }
            value={ telemetry.flight?.indicated_airspeed }/>
        <Parameter name={ $i18n.t("AMSL") } tooltip={ $i18n.t("Altitude Above Mean Sea Level, m") }
            value={ telemetry.flight?.altitude_amsl}/>

    <!-- TODO: ENGINE, RPM, FUEL? -->

    <!-- NAVIGATION -->
        <Parameter name={ $i18n.t("HDG") } tooltip={ $i18n.t("Heading") }
            value={ telemetry.flight ? formatHeading(telemetry.flight.yaw) : undefined }/>
        <div class="row-span-2 col-span-2 flex items-center justify-center">
            <HSI
                enabled={!!telemetry.flight}
                heading={telemetry.flight?.yaw || 0}
                course={telemetry.rawSns?.course || 0}
                courseEnabled={telemetry.rawSns && telemetry.rawSns.ground_speed > 1}/>
            <div id="vehicleMark" class="absolute flex items-center justify-center scale-150">
                <VehicleTypeIcon 
                    vehicleType={$selectedVehicle?.description.vehicle_type} 
                    color={$selectedVehicle?.description.color || ""}/>
            </div>
        </div>
        <Parameter name={ $i18n.t("WPT") } tooltip={ $i18n.t("Distance to next waypoint, m") }
            value={ telemetry.navigation?.wp_distance}/>
        <Parameter name={ $i18n.t("CRS") } tooltip={ $i18n.t("Course (GPS)") }
            value={ telemetry.rawSns ? formatHeading(telemetry.rawSns?.course) : undefined }/>
        <Parameter name={ $i18n.t("HOME") } tooltip={ $i18n.t("Distance to home point, m") }
            value={ $mainMap ? $mainMap.calcDistance(
                telemetry.navigation?.position, telemetry.navigation?.home_position) : undefined}/>
    </div>
    {/if}
</div>
