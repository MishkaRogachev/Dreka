<script lang="ts">
import type { System } from "$bindings/telemetry";

import { i18n } from "$stores/i18n";

import battery0Icon from "$assets/svg/battery-0.svg?raw";
import battery20Icon from "$assets/svg/battery-20.svg?raw";
import battery40Icon from "$assets/svg/battery-40.svg?raw";
import battery60Icon from "$assets/svg/battery-60.svg?raw";
import battery80Icon from "$assets/svg/battery-80.svg?raw";
import battery100Icon from "$assets/svg/battery-100.svg?raw";

export let system: System

$: percentage = Math.max(0, Math.min(100, system.battery_remaining))
$: voltage = system.battery_voltage
$: current = system.battery_current

function toBatteryIcon(percentage: number) {
    if (percentage <= 10) {
        return battery0Icon
    } else if (percentage <= 30) {
        return battery20Icon
    } else if (percentage <= 50) {
        return battery40Icon
    } else if (percentage <= 70) {
        return battery60Icon
    } else if (percentage <= 90) {
        return battery80Icon
    } else {
        return battery100Icon
    }
}

function toBatteryClass(percentage: number) {
    if (percentage <= 10) {
        return "text-error"
    } else if (percentage <= 30) {
        return "text-warning"
    } else {
        return ""
    }
}

</script>

<div class="dropdown dropdown-hover dropdown-bottom dropdown-end">
    <div tabindex="0" role="button" class={"btn m-1 fill-current " + toBatteryClass(percentage) }>
        { @html toBatteryIcon(percentage) }
    </div>
    <div tabindex="0" class="dropdown-content z-[1] p-2 w-36 shadow badge-neutral rounded-md flex flex-col align-middle">
        <p class="text-left">{ $i18n.t("Battery") + ":  " + percentage.toFixed(0) + "%" }</p>
        <p class="text-left">{ $i18n.t("Volatage") + ": " + voltage.toFixed(2) + " V"}</p>
        <p class="text-left">{ $i18n.t("Current") + ":  " + current.toFixed(2) + " A"}</p>
    </div>
</div>

<!-- // TODO: charging icon when current < 0 -->
