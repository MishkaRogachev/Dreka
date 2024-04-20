<script lang="ts">
// TODO: rename it to VehicleStateIndicator
import { VehicleState } from "$bindings/vehicles";

import { i18n } from "$stores/i18n";
import { selectedVehicle } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";

import playIcon from "$assets/svg/play.svg?raw";
import pauseIcon from "$assets/svg/pause.svg?raw";
import loadingIcon from "$assets/svg/loading.svg?raw";
import calibrateIcon from "$assets/svg/calibrate.svg?raw";
import emergencyIcon from "$assets/svg/emergency.svg?raw";
import criticalIcon from "$assets/svg/critical.svg?raw";
import unknownIcon from "$assets/svg/unknown.svg?raw";
import gpsIcon from "$assets/svg/gps.svg?raw";
import radioIcon from "$assets/svg/radio.svg?raw";
import batteryIcon from "$assets/svg/battery.svg?raw";

$: is_online = $selectedVehicle?.is_online || false
$: vehicleState = $selectedVehicle?.status?.state || VehicleState.Unknown
$: armed = $selectedVehicle?.status?.armed || false
$: readyToArm = $selectedVehicleTelemetry.system?.arm_ready || false

function toStateIcon(state: VehicleState) {
    switch (state) {
        case VehicleState.Boot:
            return loadingIcon;
        case VehicleState.Calibrating:
            return calibrateIcon;
        case VehicleState.Emergency:
            return emergencyIcon;
        case VehicleState.Critical:
            return criticalIcon;
        case VehicleState.Active:
            return playIcon;
        case VehicleState.Standby:
            return pauseIcon;
        case VehicleState.Unknown: // no break
        default:
            return unknownIcon;
    }
}

function toArmColorCode(is_online: boolean, armed: boolean, readyToArm: boolean) {
    if (!is_online) {
        return "btn-neutral"
    }
    return armed ? "btn-success" : readyToArm ? "btn-warning" : "btmn-error"
}

function toArmText(armed: boolean, readyToArm: boolean) {
    return armed ? $i18n.t("ARMED") : readyToArm ? $i18n.t("DISARMED") : $i18n.t("NOT READY")
}

</script>

<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("State") + ": " + $i18n.t(vehicleState) }>
    { @html toStateIcon(vehicleState) }
</div>
<!-- TODO: Radio RSSI & manual control indication -->
<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Radio") }>
    <button class="btn btn-sm btn-ghost btn-circle my-1" >{ @html radioIcon }</button>
</div>
<!-- TODO: Battery percentage, voltage & current indication -->
<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Battery") }>
    <button class="btn btn-sm btn-ghost btn-circle my-1" >{ @html batteryIcon }</button>
</div>
<!-- TODO: GPS state, fix, and sattelite count indication -->
<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("GPS") }>
    <button class="btn btn-sm btn-ghost btn-circle my-1" >{ @html gpsIcon }</button>
</div>
<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Open systems") }>
    <button class={ "w-22 btn btn-xs " + toArmColorCode(is_online, armed, readyToArm)}
    on:click={() => {
        // @ts-ignore
        document.getElementById("systems_modal")?.showModal();
    }}>
        { toArmText(armed, readyToArm) }
    </button>
</div>
