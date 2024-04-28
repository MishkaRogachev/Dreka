<script lang="ts">
import type { CommandExecution } from "$bindings/commands";

import { i18n } from "$stores/i18n";
import { selectedVehicle, selectedVehicleId } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";
import { commandExecutions } from "$stores/commands";

import { longpress } from "$lib/common/longpress";

import CommandDialog from "$components/dialogs/CommandDialog.svelte";
import CommandBadge from "$components/common/CommandBadge.svelte";

import lockIcon from "$assets/svg/lock.svg?raw";

$: is_online = $selectedVehicle?.is_online || false
$: armed = $selectedVehicle?.status?.armed || false
$: readyToArm = $selectedVehicleTelemetry.system?.arm_ready
$: armExecution = armToken ? $commandExecutions.get(armToken) : undefined

let armToken: string | null = null
let dalayProgress = 0;

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

function doLongpress() {
    dalayProgress += 1;
    if (dalayProgress > 100) {
        armDisarmVehicle(!armed);
    }
}

function dropLongpress() {
    dalayProgress = 0;
}

</script>

<CommandDialog>
    <h1 slot="title">{ $i18n.t("ARM/DISARM") }</h1>
    <div slot="content" class="flex gap-x-2 items-center">
        <CommandBadge state={armExecution?.state}/>

        <p class={ "w-64 " + (readyToArm ? "text-normal" : "text-error")}>
        {#if readyToArm}
            { $i18n.t("Hold button to ") }
            <b>{ (armed ? $i18n.t("DISARM") : $i18n.t("ARM")) }</b>
            {" " + $i18n.t("the vehicle") }
        {:else}
            { $i18n.t("NOT READY TO ARM") }
        {/if}
        </p>

        <div class="radial-progress border-2 border-primary btn-primary"
            style="--value:{dalayProgress}; --size:3rem;" role="progressbar"
            use:longpress={{ delay: 10, repeat: true, onLongPress: doLongpress, onDropped: dropLongpress}}>
            { @html lockIcon }
        </div>
    </div>
</CommandDialog>
