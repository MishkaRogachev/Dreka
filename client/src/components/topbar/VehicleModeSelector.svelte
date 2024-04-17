<script lang="ts">
import { VehicleMode } from '$bindings/vehicles';

import { i18n } from '$stores/i18n';
import { commandExecutions } from '$stores/commands';
import { type Vehicle, formatMode } from '$stores/vehicles';

import CommandBadge from '$components/common/CommandBadge.svelte';

export let vehicle: Vehicle;

let modeToken: string | null = null

$: currentMode = vehicle.status?.mode;
$: availableModes = vehicle.description.available_modes;
$: modeExecution = modeToken ? $commandExecutions.get(modeToken) : undefined

async function setVehicleMode(mode: VehicleMode) {
    modeToken = await commandExecutions.executeCommand(
        { SetMode: { mode: mode } },
        { Vehicle: { vehicle_id: vehicle.description.id }
    });
}

async function cancelSetVehicleMode() {
    if (modeToken) {
        await commandExecutions.cancelCommand(modeToken);
    }
}

</script>

<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Set mode") }>
    <div class="dropdown dropdown-end">
        <div tabindex="0" class="select select-ghost select-sm m-1 gap-x-2 items-center w-28">
            <a class="grow">{ formatMode(currentMode) }</a>
        </div>
        <ul tabindex="0" class="dropdown-content menu z-[1] p-0 shadow bg-base-300 rounded-md my-0">
        {#each availableModes as mode}
            <li class="w-28 flex" on:click = {() => {
                if (modeExecution?.command.SetMode?.mode === mode) {
                    cancelSetVehicleMode();
                } else {
                    setVehicleMode(mode);
                }
            }}>
                <div class="flex gap-x-2 items-center grow">
                    <a class={"grow " + (mode === currentMode ? "font-black" : "font-normal")}>
                        { formatMode(mode) }
                    </a>
                    <CommandBadge state={modeExecution?.command.SetMode?.mode === mode ? modeExecution?.state : undefined}>
                    </CommandBadge>
                </div>
            </li>
        {/each}
        </ul>
    </div>
</div>
