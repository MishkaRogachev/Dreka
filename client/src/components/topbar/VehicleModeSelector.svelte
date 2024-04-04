<script lang="ts">
import { clickOutside } from '$lib/common/click-outside';

import { VehicleMode } from '$bindings/vehicles';

import { i18n } from '$stores/i18n';
import { commandExecutions } from '$stores/commands';
import type { Vehicle } from '$stores/vehicles';

import CommandBadge from '$components/common/CommandBadge.svelte';

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

<div class="tooltip tooltip-right" data-tip={ $i18n.t("Set vehicle mode") }>
    <div id="vehicleModeSelectorDropdown" class="dropdown dropdown-end" use:clickOutside={closeDropdown}>
        <div tabindex="0" class="select select-ghost select-sm m-1 gap-x-2 items-center w-24">
            <a class="grow">{ currentMode }</a>
        </div>
        <ul tabindex="0" class="dropdown-content menu z-[1] p-0 shadow bg-base-300 rounded-md my-0">
        {#each availableModes as mode}
            <li class="btn-wide flex" on:click = {() => { setVehicleMode(mode); closeDropdown(); }}>
                <div class="flex gap-x-2 items-center grow">
                    <a class={"grow " + (mode === currentMode ? "text-white" : "")}>
                        { mode }
                    </a>
                    <CommandBadge state={modeExecution?.command.SetMode?.mode === mode ? modeExecution?.state : undefined}>
                    </CommandBadge>
                </div>
            </li>
        {/each}
        </ul>
    </div>
</div>