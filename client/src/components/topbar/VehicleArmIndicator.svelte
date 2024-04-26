<script lang="ts">
import { i18n } from "$stores/i18n";
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
</script>

<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Open systems") }>
    <button class={ "w-22 btn btn-xs " + toArmColorCode(is_online, armed, readyToArm)}
    on:click={() => {
        // @ts-ignore
        document.getElementById("systems_modal")?.showModal();
    }}>
        { toArmText(armed, readyToArm) }
    </button>
</div>
