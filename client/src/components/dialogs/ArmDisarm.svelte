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

$: armed = $selectedVehicle?.status?.armed || false
$: readyToArm = $selectedVehicleTelemetry.system?.arm_ready
$: armExecution = armToken ? $commandExecutions.get(armToken) : undefined

let armToken: string | null = null
let dalayProgress = 0;
let sent: boolean = false;

async function armDisarmVehicle(arm: boolean) {
    armToken = await commandExecutions.executeCommand(
        { ArmDisarm: { arm: arm } },
        { Vehicle: { vehicle_id: $selectedVehicleId }
    });
}

function doLongpress() {
    if (sent) {
        return;
    }

    dalayProgress += 1;
    if (dalayProgress > 100) {
        armDisarmVehicle(!armed);
        dalayProgress = 0;
        sent = true;
    }
}

function dropLongpress() {
    dalayProgress = 0;
    sent = false;
}

</script>

<CommandDialog>
    <h1 slot="title">{ $i18n.t("ARM/DISARM") }</h1>
    <div slot="content" class="flex gap-x-2 items-center">
        <CommandBadge state={armExecution?.state}/>

        <p class="w-64">
            { $i18n.t("Hold button to ") }
            <b>{ (armed ? $i18n.t("DISARM") : $i18n.t("ARM")) }</b>
            { " " + $i18n.t("the vehicle") }
        </p>

        <div class="tooltip tooltip-bottom" data-tip={
            readyToArm ? armed ? $i18n.t("Disarm vehicle") : $i18n.t("Arm vehicle") : $i18n.t("Not ready") }>
            <div class={"radial-progress border-2 " + (readyToArm ? "border-primary btn-primary" : "border-error btn-error")}
                style="--value:{dalayProgress}; --size:3rem;" role="progressbar"
                use:longpress={{ delay: 10, repeat: true, onLongPress: doLongpress, onDropped: dropLongpress}}>
                { @html lockIcon }
            </div>
        </div>
    </div>
</CommandDialog>
