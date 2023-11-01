<script lang="ts">
import Ai from "$components/indicators/AI.svelte";
import Hsi from "$components/indicators/HSI.svelte";
import Parameter from "$components/indicators/Parameter.svelte";
import VehicleTypeIcon from "$components/common/VehicleTypeIcon.svelte";

import { selectedVehicle } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";
import { i18n } from '$stores/i18n';

import { degreesToDmsString, formatHeading } from "$lib/common/formats";

import centerIcon from "$assets/svg/center.svg?raw";
import switchIcon from "$assets/svg/switch.svg?raw";

let dms: boolean = true

$: telemetry = $selectedVehicleTelemetry
$: online = $selectedVehicle?.is_online()

$: latitude = dms ? degreesToDmsString(telemetry.flight.position.latitude, false)
                : telemetry.flight.position.latitude.toFixed(6)
$: longitude = dms ? degreesToDmsString(telemetry.flight.position.longitude, true)
                : telemetry.flight.position.longitude.toFixed(6)

function coordsToClipboard() { navigator.clipboard.writeText(latitude + " " + longitude) }

function switchVehicleTracking() {

}

</script>

<div id="dashboard" class="absolute top-10 right-2 bg-base-200 p-4 rounded-md shadow-lg">
    <div class = "grid grid-cols-4 gap-2 text-center items-center justify-items-center">
    <!-- COORDINATES & TRACK VERHICLE -->
    <div class="col-span-4 flex place-self-stretch join">
        <button class="btn btn-sm px-1 join-item" on:click={switchVehicleTracking}>{@html centerIcon}</button>
        <button class="btn btn-sm px-1 join-item grow font-mono text-right" on:click={coordsToClipboard}>
            {latitude} <br/> {longitude}
        </button>
        <button class="btn btn-sm px-1 join-item" on:click={() => { dms = !dms }}>{@html switchIcon}</button>
    </div>
    <!-- FLIGHT DATA DISPLAY -->
        <Parameter name={ $i18n.t("GS") } value={telemetry.sns.ground_speed}/>
        <div class="row-span-3 col-span-2">
            <Ai online={online} pitch={telemetry.flight.pitch} roll={telemetry.flight.roll}/>
        </div>
        <Parameter name={ $i18n.t("ASNS") } value={ telemetry.sns.position.altitude }/>
        <Parameter name={ $i18n.t("IAS") } value={ telemetry.flight.indicated_airspeed }/>
        <Parameter name={ $i18n.t("AMSL") } value={ telemetry.flight.altitude_amsl }/>
        <a class="text-sm">{ $i18n.t("m/s") }</a>
        <a class="text-sm">{ $i18n.t("m") }</a>

    <!-- NAVIGATION DATA DISPLAY -->
        <Parameter name={ $i18n.t("HDG") } value={ formatHeading(telemetry.flight.yaw) }/>
        <div class="row-span-3 col-span-2 relative">
            <Hsi heading={telemetry.flight.yaw} course={telemetry.sns.course}/>
            <div id="vehicleMark" class="absolute inset-0 flex items-center justify-center scale-150">
                <VehicleTypeIcon vehicleType={$selectedVehicle?.description.vehicle_type}/>
            </div>
        </div>
        <Parameter name={ $i18n.t("WPT") } value={ telemetry.flight.wp_distance }/>
        <Parameter name={ $i18n.t("CRS") } value={ formatHeading(telemetry.sns.course) }/>
        <Parameter name={ $i18n.t("HOME") } value={ telemetry.distanceToHome() }/>
        <a class="text-sm">&deg</a>
        <a class="text-sm">{ $i18n.t("m") }</a>
    </div>
</div>