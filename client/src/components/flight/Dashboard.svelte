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
$: online = $selectedVehicle?.is_online

$: latitude = telemetry.flight ? (dms ? degreesToDmsString(telemetry.flight.position.latitude, false)
                : telemetry.flight.position.latitude.toFixed(6)) : $i18n.t("N/A")
$: longitude = telemetry.flight ? (dms ? degreesToDmsString(telemetry.flight.position.longitude, true)
                : telemetry.flight.position.longitude.toFixed(6)) : $i18n.t("N/A")

function coordsToClipboard() { navigator.clipboard.writeText(latitude + " " + longitude) }

function switchVehicleTracking() {} // TODO: center/track vehicle

</script>

<div id="dashboard" class="absolute top-10 right-2 bg-base-200 p-4 rounded-md shadow-lg">
    <div class = "grid grid-cols-4 gap-2 text-center items-center justify-items-stretch">
    <!-- COORDINATES & TRACK VERHICLE -->
    <div class="col-span-4 join content-center">
        <div class="tooltip tooltip-left" data-tip={ $i18n.t("Center vehicle on map") }>
            <button class="btn btn-sm px-1 join-item h-full" on:click={switchVehicleTracking}>{@html centerIcon}</button>
        </div>
        <div class="tooltip tooltip-left grow" data-tip={ $i18n.t("Vehicle coordinates, click to copy") }>
            <button class="btn btn-sm px-1 join-item font-mono text-right h-full" on:click={coordsToClipboard}>
                {latitude} <br/> {longitude}
            </button>
        </div>
        <div class="tooltip tooltip-bottom" data-tip={ $i18n.t("DMS/D.D") }>
            <button class="btn btn-sm px-1 join-item h-full" on:click={() => { dms = !dms }}>{@html switchIcon}</button>
        </div>
    </div>
    <!-- FLIGHT DATA DISPLAY -->
        <Parameter name={ $i18n.t("GS") } tooltip={ $i18n.t("Ground (GPS) Speed") }
            value={telemetry.navigation ? telemetry.navigation.ground_speed : 0}/>
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
        <a class="text-sm">{ $i18n.t("m/s") }</a>
        <a class="text-sm">{ $i18n.t("m") }</a>

    <!-- NAVIGATION DATA DISPLAY -->
        <Parameter name={ $i18n.t("HDG") } tooltip={ $i18n.t("Heading") }
            value={ formatHeading(telemetry.flight ? telemetry.flight.yaw : 0) }/>
        <div class="row-span-3 col-span-2 relative">
            <Hsi
                heading={telemetry.flight ? telemetry.flight.yaw : 0}
                course={telemetry.navigation ? telemetry.navigation.course : 0}
            />
            <div id="vehicleMark" class="absolute inset-0 flex items-center justify-center scale-150">
                <VehicleTypeIcon 
                    vehicleType={$selectedVehicle?.description.vehicle_type} 
                    color={$selectedVehicle?.description.color || ""}/>
            </div>
        </div>
        <Parameter name={ $i18n.t("WPT") } tooltip={ $i18n.t("Distance to next waypoint") }
            value={ telemetry.flight ? telemetry.flight.wp_distance : 0 }/>
        <Parameter name={ $i18n.t("CRS") } tooltip={ $i18n.t("Course (GPS)") }
            value={ telemetry.navigation ? formatHeading(telemetry.navigation.course) : 0 }/>
        <Parameter name={ $i18n.t("HOME") } tooltip={ $i18n.t("Distance to home point") }
            value={ 0 }/> <!-- TODO: home distance -->
        <a class="text-sm">&deg</a>
        <a class="text-sm">{ $i18n.t("m") }</a>
    </div>
</div>