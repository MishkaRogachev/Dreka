<script lang="ts">
import { Calibration } from "$bindings/commands";

import { i18n } from "$stores/i18n";
import { commandExecutions } from "$stores/commands";
import { selectedVehicleId } from "$stores/vehicles";

import CommandDialog from "$components/dialogs/CommandDialog.svelte";
import CommandBadge from "$components/common/CommandBadge.svelte";

$: calibrateExecution = calibrateToken ? $commandExecutions.get(calibrateToken) : undefined

let calibrateToken: string | null = null

async function calibrate(calibration: Calibration) {
    calibrateToken = await commandExecutions.executeCommand(
        { Calibrate: { calibration: calibration} },
        { Vehicle: { vehicle_id: $selectedVehicleId }
    });
}

</script>

<CommandDialog>
    <div slot="title" class="flex gap-2 items-center">
        <CommandBadge state={calibrateExecution?.state}/>
        { $i18n.t("PREFLIGHT CALIBRATIONS") }
    </div>

    <div slot="content" class="grid grid-cols-2 gap-2 max-scroll-area-height overflow-y-auto">
        <p>{ $i18n.t("Airspeed sensor") }</p>
        <button class="btn btn-sm btn-primary" on:click={() => { calibrate(Calibration.Airspeed) }}>
            { $i18n.t("Calibrate") }
        </button>

        <p>{ $i18n.t("Ground pressure") }</p>
        <button class="btn btn-sm btn-primary" on:click={() => { calibrate(Calibration.GroundPressure) }}>
            { $i18n.t("Calibrate") }
        </button>

        <p>{ $i18n.t("Temperature") }</p>
        <button class="btn btn-sm btn-primary" on:click={() => { calibrate(Calibration.Temperature) }}>
            { $i18n.t("Calibrate") }
        </button>

        <p>{ $i18n.t("Board level") }</p>
        <button class="btn btn-sm btn-warning" on:click={() => { calibrate(Calibration.BoardLevel) }}>
            { $i18n.t("RESET") }
        </button>
    </div>
</CommandDialog>
