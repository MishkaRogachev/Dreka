<script lang="ts">
import VehicleModeSelector from "$components/topbar/VehicleModeSelector.svelte";

import { VehicleState } from "$bindings/vehicles";

import { i18n } from "$stores/i18n";
import { selectedVehicle } from "$stores/vehicles";

$: is_online = $selectedVehicle?.is_online || false
$: vehicleState = $selectedVehicle?.status?.state || VehicleState.Unknown
$: armed = $selectedVehicle?.status?.armed || false
$: readyToArm = true // TODO: SYS_STATUS

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

<div class="flex items-center gap-x-2 font-bold">
    <VehicleModeSelector vehicle={ $selectedVehicle }/>
    <a class="w-12 text-xs">{ $i18n.t(vehicleState) }</a>
    <span class={"badge badge-xs " + (is_online ? "bg-success" : "bg-neutral-content")} ></span>
    <button class={ "btn btn-xs " + toArmColorCode(is_online, armed, readyToArm)} on:click={() => {
        // @ts-ignore
        document.getElementById("flight_checks_modal")?.showModal();
    }}>{ toArmText(armed, readyToArm) }</button>
</div>