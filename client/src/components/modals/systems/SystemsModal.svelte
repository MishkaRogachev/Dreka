<script lang="ts">
import CommandButton from '$components/common/CommandButton.svelte';

import BaseModal from "$components/common/BaseModal.svelte";
import SensorHealth from '$components/modals/systems/SensorHealth.svelte';

import { i18n } from "$stores/i18n";
import { selectedVehicle, selectedVehicleId } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";
import { commandExecutions } from '$stores/commands';

import alertIcon from "$assets/svg/alert.svg?raw";

$: is_online = $selectedVehicle?.is_online || false
$: armed = $selectedVehicle?.status?.armed || false
$: readyToArm = $selectedVehicleTelemetry.system?.arm_ready
$: armExecution = armToken ? $commandExecutions.get(armToken) : undefined

$: sensors = $selectedVehicleTelemetry.system?.sensors || []

let armToken: string | null = null

async function armDisarmVehicle(arm: boolean) {
    armToken = await commandExecutions.executeCommand(
        { ArmDisarm: { arm: arm } },
        { Vehicle: { vehicle_id: $selectedVehicleId }
    });
}

async function cancelArmDisarm() {
    if (armToken) {
        await commandExecutions.cancelCommand(armToken);
    }
}

</script>

<style>
.max-scroll-area-height {
    max-height: 30vh;
}
</style>

<BaseModal id="systems_modal">
    <form method="dialog">
        <!-- CLOSE -->
        <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
    </form>
    <h3 class="font-bold text-lg text-center mb-4">{ $i18n.t("Systems Panel") }</h3>

    <!-- CHECKS LIST COMPONENT -->
    <div class="grid grid-cols-2 gap-4 max-scroll-area-height overflow-y-auto">
        {#each sensors as sensor}
        <SensorHealth sensor={sensor}/>
        {/each}
    </div>

    <!-- FILLER -->
    <div class="flex flex-col grow text-center">
    {#if sensors.length === 0}
        <a href={null} class="grow">{ $i18n.t("No sensor data available") }</a>
    {:else}
        <div class="grow"/>
    {/if}
    </div>

    <div class="divider">
        <label class="label cursor-pointer gap-x-2">
            <span class="label-text">{ $i18n.t("DANGER ZONE") }</span>
        </label>
    </div>

    <div role="alert" class={"alert"}>
        {@html alertIcon}
        <!-- (readyToArm ? "" : " alert-warning") -->
        <div>
            <h3 class="font-bold">{ armed ?
                "Vehicle is ARMED" :
                "Vehicle is DISARMED"}
            </h3>
            <div class="text-s">{ armed ?
                "Disarming vehicle in flight can lead to a crash!" :
                "Be careful when arming the vehicle!"}
            </div>
        </div>
        <div>
            <CommandButton btnClass="btn btn-wide btn-warning"
                disabled={!is_online} state={armExecution?.state}
                on:execute={() => armDisarmVehicle(!armed)} on:cancel={() => cancelArmDisarm()}>
                { armed ? $i18n.t("DISARM VEHICLE") : $i18n.t("ARM VEHICLE") }
            </CommandButton>
        </div>
    </div>
</BaseModal>
