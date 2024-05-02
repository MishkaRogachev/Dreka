<script lang="ts">
import { i18n } from "$stores/i18n";
import { selectedVehicle, selectedVehicleId } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";
import { commandExecutions } from "$stores/commands";

import { longpress } from "$lib/common/longpress";

import BaseDialog from "$components/dialogs/BaseDialog.svelte";
import CommandBadge from "$components/common/CommandBadge.svelte";

import lockIcon from "$assets/svg/lock.svg?raw";

$: armed = $selectedVehicle?.status?.armed || false;
$: readyToArm = $selectedVehicleTelemetry.system?.arm_ready || false;
$: armExecution = armToken ? $commandExecutions.get(armToken) : undefined;

let armToken: string | null = null;
let dalayProgress = 0;
let sent: boolean = false;
let closeDialog: () => void;

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

function toArmingText(armed: boolean, readyToArm: boolean) {
    if (!readyToArm) {
        return $i18n.t("Not ready to arm");
    }
    return armed ?
        $i18n.t("Disarming in flight may cause the vehicle to crash") :
        $i18n.t("Arming may cause engine to start and vehicle to takeoff");
}

</script>

<BaseDialog bind:closeDialog={closeDialog}>
    <div slot="title" class="flex gap-2 items-center">
        <CommandBadge state={armExecution?.state} on:succeeded={() => { closeDialog(); }} />
        { armed ? $i18n.t("DISARM VEHICLE") : $i18n.t("ARM VEHICLE") }
    </div>
    <div slot="content" class="flex gap-x-2 items-center">
        <p class={ "w-64 " + (readyToArm ? "" : "text-warning") } >{ toArmingText(armed, readyToArm) }</p>
        <div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Hold to") + " " + (armed ? $i18n.t("disarm") : $i18n.t("arm")) }>
            <div class={"radial-progress border-2 " + (readyToArm ? "border-primary btn-primary" : "border-warning btn-warning")}
                style="--value:{dalayProgress}; --size:3rem;" role="progressbar"
                use:longpress={{ delay: 10, repeat: true, onLongPress: doLongpress, onDropped: dropLongpress}}>
                { @html lockIcon }
            </div>
        </div>
    </div>
</BaseDialog>
