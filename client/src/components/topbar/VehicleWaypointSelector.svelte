<script lang="ts">
import { i18n } from '$stores/i18n';
import { commandExecutions } from '$stores/commands';
import type { Vehicle } from '$stores/vehicles';
import { formatRouteItem, selectedVehicleMission } from '$stores/mission';

import CommandBadge from '$components/common/CommandBadge.svelte';
import Dropdown from '$components/map/common/Dropdown.svelte';

export let vehicle: Vehicle;

let closeDropdown: () => void;
let wptToken: string | null = null;

$: availableWayponts = $selectedVehicleMission?.route.items.map(item => item.type) || [];
$: currentWptIndex = $selectedVehicleMission?.status.progress.current;
$: currentWptType = availableWayponts[currentWptIndex || 0];
$: wptExecution = wptToken ? $commandExecutions.get(wptToken) : undefined;

async function setWaypoint(wpt: number) {
    wptToken = await commandExecutions.executeCommand(
        { SetWaypoint: { wpt: wpt + 1 } },
        { Vehicle: { vehicle_id: vehicle.description.id }
    });
}

</script>

<Dropdown bind:closeDropdown={closeDropdown} tip={ $i18n.t("Set waypoint") }>
    <div slot="summary" class="flex w-20 gap-x-2 items-center">
        <CommandBadge state={wptExecution?.state}/>
        <span class = "uppercase text-sm">{ formatRouteItem(currentWptType, currentWptIndex) }</span>
    </div>
    <ul slot="details" class="menu p-0">
    {#each availableWayponts as waypoint, wpt}
        <li class="flex" on:click = {() => { setWaypoint(wpt); }}>
            <div class="flex w-28 gap-x-2 items-center">
                <span class={"grow " + (wpt === currentWptIndex ? "font-black" : "font-normal")}>
                    { formatRouteItem(waypoint, wpt) }
                </span>
            </div>
        </li>
    {/each}
    </ul>
</Dropdown>

<!-- <Dropdown bind:closeDropdown={closeDropdown} tip={ $i18n.t("Set mode") }>
    <div slot="summary" class="flex w-24 gap-x-2 items-center">
        <CommandBadge state={modeExecution?.state}/>
        <span class = "uppercase text-sm">{ formatMode(currentMode) }</span>
    </div>
    <ul slot="details" class="menu p-0">
    {#each availableModes as mode}
        <li class="flex" on:click = {() => { setVehicleMode(mode); }}>
            <div class="flex w-32 gap-x-2 uppercase items-center">
                <a href={null} class={"uppercase " + (mode === currentMode ? "font-black" : "font-normal")}>
                    { formatMode(mode) }
                </a>
            </div>
        </li>
    {/each}
    </ul>
</Dropdown> -->
