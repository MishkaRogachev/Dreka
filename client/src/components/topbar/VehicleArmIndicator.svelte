<script lang="ts">
import { i18n } from "$stores/i18n";
import { activeDialog } from "$stores/app";
import { selectedVehicle } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";

$: is_online = $selectedVehicle?.is_online || false
$: armed = $selectedVehicle?.status?.armed || false
$: readyToArm = $selectedVehicleTelemetry.system?.arm_ready || false

function toArmColorCode(is_online: boolean, armed: boolean, readyToArm: boolean) {
    if (!is_online) {
        return "btn-neutral"
    }
    return armed ? "btn-success" : readyToArm ? "btn-warning" : "btmn-error"
}

function toArmText(armed: boolean, readyToArm: boolean) {
    return armed ? $i18n.t("ARMED") : readyToArm ? $i18n.t("DISARMED") : $i18n.t("NOT READY")
}

async function openArmDisarm() {
    $activeDialog = (await import('$components/dialogs/ArmDisarm.svelte')).default;
}
</script>

<div class="tooltip tooltip-bottom" data-tip={ armed ? $i18n.t("DISARM") : $i18n.t("ARM") }>
    <button class={ "w-22 btn btn-xs " + toArmColorCode(is_online, armed, readyToArm)}
        on:click={openArmDisarm}>
        { toArmText(armed, readyToArm) }
    </button>
</div>
