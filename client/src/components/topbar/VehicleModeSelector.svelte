<script lang="ts">
import { VehicleMode } from '$bindings/vehicles';

import { i18n } from '$stores/i18n';
import { commandExecutions } from '$stores/commands';
import { type Vehicle, formatMode } from '$stores/vehicles';

import CommandBadge from '$components/common/CommandBadge.svelte';
import Dropdown from '$components/common/Dropdown.svelte';

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
    closeDropdown();
}

</script>

<Dropdown bind:closeDropdown={closeDropdown} tip={ $i18n.t("Set mode") }>
    <div slot="summary" class="flex w-24 gap-x-2 items-center">
        <CommandBadge state={modeExecution?.state}/>
        <span class = "uppercase text-sm">{ formatMode(currentMode) }</span>
    </div>
    <ul slot="details" class="menu p-0">
    {#each availableModes as mode}
        <li class="flex" on:click = {() => { setVehicleMode(mode); }}>
            <div class="flex w-32 gap-x-2 uppercase items-center">
                <span class={"uppercase " + (mode === currentMode ? "font-black" : "font-normal")}>
                    { formatMode(mode) }
                </span>
            </div>
        </li>
    {/each}
    </ul>
</Dropdown>
