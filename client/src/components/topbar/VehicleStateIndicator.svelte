<script lang="ts">
import { VehicleState, type VehicleStatus } from "$bindings/vehicles";

import { i18n } from "$stores/i18n";

import playIcon from "$assets/svg/play.svg?raw";
import pauseIcon from "$assets/svg/pause.svg?raw";
import loadingIcon from "$assets/svg/loading.svg?raw";
import calibrateIcon from "$assets/svg/calibrate.svg?raw";
import emergencyIcon from "$assets/svg/emergency.svg?raw";
import criticalIcon from "$assets/svg/critical.svg?raw";
import unknownIcon from "$assets/svg/unknown.svg?raw";

export let vehicleStatus: VehicleStatus | undefined;

function toStateIcon(state: VehicleState | undefined) {
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

function toStateClass(state: VehicleState | undefined) {
    switch (state) {
        case VehicleState.Boot:
            return "text-secondary";
        case VehicleState.Calibrating:
            return "text-info";
        case VehicleState.Emergency:
            return "text-warning";
        case VehicleState.Critical:
            return "text-error";
        case VehicleState.Active:
        case VehicleState.Standby:
            return "text-base";
        case VehicleState.Unknown: // no break
        default:
            return "text-neutral";
    }
}
</script>

<div class="dropdown dropdown-hover dropdown-bottom dropdown-end">
    <div tabindex="0" role="button" class={"btn-xs fill-current " + toStateClass(vehicleStatus?.state)}>
        { @html toStateIcon(vehicleStatus?.state) }
    </div>
    <div tabindex="0" class="dropdown-content z-[1] p-2 w-36 shadow badge-neutral rounded-md flex flex-col align-middle">
        <div class="flex justify-between">
            <div class="text-left">{ $i18n.t("State") + ":" }</div>
            <div class="text-right">{ vehicleStatus ? $i18n.t(vehicleStatus.state) : "-" }</div>
        </div>
    </div>
</div>
