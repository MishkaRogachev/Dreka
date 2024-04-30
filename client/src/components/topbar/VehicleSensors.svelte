<script lang="ts">
import type { Sensor, System } from "$bindings/telemetry";

import { i18n } from "$stores/i18n";

import sensorsIcon from "$assets/svg/sensor.svg?raw";

export let system: System | undefined

$: sensors = system?.sensors || []

function toSensorsClass(sensors: Sensor[]) {
    let summary = "text-neutral";
    for (const sensor of sensors) {
        if (!sensor.enabled) {
            continue;
        }
        if (!sensor.health) {
            summary = "text-error";
            break;
        } else {
            summary = "";
        }
    }
    return summary;
}

</script>

<div class="dropdown dropdown-hover dropdown-bottom dropdown-end">
    <div tabindex="0" role="button" class={"btn-xs fill-current " + toSensorsClass(sensors)}>
        { @html sensorsIcon }
    </div>
    <div tabindex="0" class="dropdown-content z-[1] p-2 gap-2 w-64 shadow-lg bg-base-100 rounded-md flex flex-col align-middle">
        <p class="text-center font-bold">{ $i18n.t("Sensors") }</p>
        <div class="grid grid-cols-2 gap-2 max-scroll-area-height overflow-y-auto">
            {#each sensors as sensor}
            <div class={"badge badge-md w-full " + (sensor.enabled ? (sensor.health ? "badge-outline" : "badge-error") : "badge-ghost" )}>
                <!-- TODO: sensor icons -->
                { sensor.name }
            </div>
            {/each}
        </div>
    </div>
</div>
