<script lang="ts">
import { VehicleMode } from '$bindings/vehicles';

import { i18n } from '$stores/i18n';
import { commandExecutions } from '$stores/commands';
import { type Vehicle, formatMode } from '$stores/vehicles';

import CommandBadge from '$components/common/CommandBadge.svelte';
import Dropdown from '$components/map/common/Dropdown.svelte';

export let vehicle: Vehicle;

let closeDropdown: () => void;
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

<Dropdown bind:closeDropdown={closeDropdown} tip={ $i18n.t("Set mode") }>
    <div slot="summary" class="flex gap-x-2 items-center text-sm w-20">
        <span class="uppercase">{ formatMode(currentMode) }</span>
    </div>
    <ul slot="details" class="menu p-0">
    {#each availableModes as mode}
        <li class="w-32 flex" on:click = {() => {
            if (modeExecution?.command.SetMode?.mode === mode) {
                cancelSetVehicleMode();
            } else {
                setVehicleMode(mode);
            }
        }}>
            <div class="flex gap-x-2 items-center grow">
                <a href={null} class={"grow uppercase " + (mode === currentMode ? "font-black" : "font-normal")}>
                    { formatMode(mode) }
                </a>
                <CommandBadge state={modeExecution?.command.SetMode?.mode === mode ? modeExecution?.state : undefined}>
                </CommandBadge>
            </div>
        </li>
    {/each}
    </ul>
</Dropdown>
