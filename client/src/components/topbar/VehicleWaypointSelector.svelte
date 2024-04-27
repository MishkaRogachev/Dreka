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

async function cancelSetWaypoint() {
    if (wptToken) {
        await commandExecutions.cancelCommand(wptToken);
    }
}
</script>

<Dropdown bind:closeDropdown={closeDropdown} tip={ $i18n.t("Set waypoint") }>
    <div slot="summary" class="flex gap-x-2 items-center text-sm w-16">
        <span>{ formatRouteItem(currentWptType, currentWptIndex) }</span>
    </div>
    <ul slot="details" class="menu p-0">
    {#each availableWayponts as waypoint, wpt}
        <li class="w-28 flex" on:click = {() => {
            if (wptExecution?.command.SetWaypoint?.wpt === wpt + 1) {
                cancelSetWaypoint();
            } else {
                setWaypoint(wpt);
            }
        }}>
            <div class="flex gap-x-2 items-center grow">
                <a href={null} class={"grow " + (wpt === currentWptIndex ? "font-black" : "font-normal")}>
                    { formatRouteItem(waypoint, wpt) }
                </a>
                <CommandBadge state={wptExecution?.command.SetWaypoint?.wpt === wpt + 1 ? wptExecution?.state : undefined}>
                </CommandBadge>
            </div>
        </li>
    {/each}
    </ul>
</Dropdown>

