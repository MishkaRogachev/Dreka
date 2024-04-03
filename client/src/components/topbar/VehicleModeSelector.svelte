<script lang="ts">
import { clickOutside } from '$lib/common/click-outside';

import { VehicleMode } from '$bindings/vehicles';

import { i18n } from '$stores/i18n';
import { commandExecutions } from '$stores/commands';
import type { Vehicle } from '$stores/vehicles';

export let vehicle: Vehicle | undefined;

$: currentMode = vehicle?.status?.mode || VehicleMode.None;
$: availableModes = vehicle?.description.available_modes || [];
$: modeExecution = modeToken ? $commandExecutions.get(modeToken) : undefined

let modeToken: string | null = null

async function setVehicleMode(mode: VehicleMode) {
    modeToken = await commandExecutions.executeCommand(
        { SetMode: { mode: mode } },
        { Vehicle: { vehicle_id: vehicle?.description.id || "" }
    });
}

async function cancelArmDismode() {
    if (modeToken) {
        await commandExecutions.cancelCommand(modeToken);
    }
}

function closeDropdown() {
    document.getElementById("vehicleModeSelectorDropdown")?.removeAttribute("open");
}

</script>

<details id="vehicleModeSelectorDropdown" class="dropdown dropdown-end" use:clickOutside={closeDropdown}>
    <summary class="select select-ghost select-xs m-1 gap-x-2 items-center">
        <a class="grow">{ currentMode }</a>
    </summary>
    <ul class="dropdown-content menu z-[1] p-0 shadow bg-base-300 rounded-md my-0">
    {#each availableModes as mode}
        <li class={"btn-wide flex " + (currentMode === mode ? "text-white" : "")}
            on:click = {() => { setVehicleMode(mode); closeDropdown(); }}>
            <div class="flex gap-x-2 items-center grow">
                <a class="grow">{ mode }</a>
            </div>
        </li>
    {/each}
    </ul>
</details>
