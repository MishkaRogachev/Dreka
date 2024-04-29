<script lang="ts">
import { i18n } from "$stores/i18n";
import { activeDialog } from "$stores/app";
import { selectedVehicle } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";

import ArmDisarm from "$components/dialogs/ArmDisarm.svelte";

$: isOnline = $selectedVehicle?.is_online || false
$: armed = $selectedVehicle?.status?.armed || false
$: readyToArm = $selectedVehicleTelemetry.system?.arm_ready || false
$: isArmDialogOpen = $activeDialog === ArmDisarm

function toArmColorCode(isOnline: boolean, armed: boolean, readyToArm: boolean) {
    return !isOnline ? "btn-active btn-neutral" : armed ? "btn-success" : readyToArm ? "btn-warning" : "btmn-error"
}

function toArmText(armed: boolean, readyToArm: boolean) {
    return !isOnline ? $i18n.t("OFFLINE") : armed ? $i18n.t("ARMED") : readyToArm ? $i18n.t("DISARMED") : $i18n.t("NOT READY")
}

async function openCloseArmDisarm() {
    $activeDialog = isArmDialogOpen ? undefined : ArmDisarm;
}
</script>

<div class="tooltip tooltip-left" data-tip={
    (isArmDialogOpen ? $i18n.t("Close") : $i18n.t("Open")) + " " + $i18n.t("ARM/DISARM dialog")}>
    <button class={ "w-22 btn btn-xs " + toArmColorCode(isOnline, armed, readyToArm)}
        on:click={openCloseArmDisarm}>
        { toArmText(armed, readyToArm) }
    </button>
</div>
