<script lang="ts">
import { MissionRouteItemType } from '$bindings/mission';

import { i18n } from '$stores/i18n';
import { commandExecutions } from '$stores/commands';
import type { Vehicle } from '$stores/vehicles';
import { formatRouteItem, selectedVehicleMission } from '$stores/mission';

import CommandBadge from '$components/common/CommandBadge.svelte';

export let vehicle: Vehicle;

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

<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Set waypoint") }>
    <div class="dropdown dropdown-end">
        <div tabindex="0" class="select select-ghost select-sm m-1 gap-x-2 items-center w-28">
            <a class="grow">{ formatRouteItem(currentWptType, currentWptIndex) }</a>
        </div>
        <div tabindex="0" class="dropdown-content menu z-[1] p-0 shadow bg-base-300
            rounded-md max-scroll-area-height overflow-y-auto max-h-96">
            <ul class="my-0">
            {#each availableWayponts as waypoint, wpt}
                <li class="w-28 flex" on:click = {() => {
                    if (wptExecution?.command.SetWaypoint?.wpt === wpt + 1) {
                        cancelSetWaypoint();
                    } else {
                        setWaypoint(wpt);
                    }
                }}>
                    <div class="flex gap-x-2 items-center grow">
                        <a class={"grow " + (wpt === currentWptIndex ? "font-black" : "font-normal")}>
                            { formatRouteItem(waypoint, wpt) }
                        </a>
                        <CommandBadge state={wptExecution?.command.SetWaypoint?.wpt === wpt + 1 ? wptExecution?.state : undefined}>
                        </CommandBadge>
                    </div>
                </li>
            {/each}
            </ul>
        </div>
    </div>
</div>
