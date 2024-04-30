<script lang="ts">
import type { System } from "$bindings/telemetry";

import { i18n } from "$stores/i18n";

import radioIcon from "$assets/svg/radio.svg?raw";

export let system: System | undefined
// TODO: export let sensor: Sensor

function toRadioClass(system?: System) {
    if (!system || system.radio_rssi == 0) {
        return "text-neutral"
    } else {
        return ""
    }
}

</script>

<div class="dropdown dropdown-hover dropdown-bottom dropdown-end">
    <div tabindex="0" role="button" class={"btn-xs fill-current " + toRadioClass(system)}>
        { @html radioIcon }
    </div>
    <div tabindex="0" class="dropdown-content z-[1] p-2 gap-2 w-36 shadow-lg bg-base-100 rounded-md flex flex-col align-middle">
        <p class="text-center font-bold">{ $i18n.t("Radio Control") }</p>
        <div class="flex justify-between">
            <div class="text-left">{ $i18n.t("Local RSSI") + ":" }</div>
            <div class="text-right">{ system ? system.radio_rssi.toFixed(0) : "-" }</div>
        </div>
        <div class="flex justify-between">
            <div class="text-left">{ $i18n.t("Remote RRSI") + ":" }</div>
            <div class="text-right">{ system ? system.radio_remote_rssi.toFixed(0) : "-" }</div>
        </div>
    </div>
</div>
