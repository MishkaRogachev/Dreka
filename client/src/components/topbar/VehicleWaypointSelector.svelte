<script lang="ts">
import { i18n } from '$stores/i18n';
import { commandExecutions } from '$stores/commands';
import type { Vehicle } from '$stores/vehicles';

import CommandBadge from '$components/common/CommandBadge.svelte';

export let vehicle: Vehicle | undefined;

$: currentWaypont = 0 // TODO: currentWaypoint
$: availableWayponts = ["HOME"] // TODO: currentWaypoint
$: wptExecution = wptToken ? $commandExecutions.get(wptToken) : undefined

let wptToken: string | null = null

async function setWaypoint(wpt: number) {
    wptToken = await commandExecutions.executeCommand(
        { SetWaypoint: { wpt: wpt } },
        { Vehicle: { vehicle_id: vehicle?.description.id || "" }
    });
}

async function cancelSetWaypoint() {
    if (wptToken) {
        await commandExecutions.cancelCommand(wptToken);
    }
}

function formatWpt(waypoint: String, wpt: number) {
    return waypoint + " " + wpt;
}

</script>

<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Set waypoint") }>
    <div class="dropdown dropdown-end">
        <div tabindex="0" class="select select-ghost select-sm m-1 gap-x-2 items-center w-32">
            <a class="grow">{ formatWpt(availableWayponts[currentWaypont], currentWaypont) }</a>
        </div>
        <ul tabindex="0" class="dropdown-content menu z-[1] p-0 shadow bg-base-300 rounded-md my-0">
        {#each availableWayponts as waypoint, wpt}
            <li class="w-32 flex" on:click = {() => {
                if (wptExecution?.command.SetWaypoint?.wpt === wpt) {
                    cancelSetWaypoint();
                } else {
                    setWaypoint(wpt);
                }
            }}>
                <div class="flex gap-x-2 items-center grow">
                    <a class={"grow " + (wpt === currentWaypont ? "text-white" : "")}>
                        { formatWpt(waypoint, wpt) }
                    </a>
                    <CommandBadge state={wptExecution?.command.SetWaypoint?.wpt === wpt ? wptExecution?.state : undefined}>
                    </CommandBadge>
                </div>
            </li>
        {/each}
        </ul>
    </div>
</div>
