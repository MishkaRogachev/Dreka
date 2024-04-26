<script lang="ts">
import type { RawSns } from "$bindings/telemetry";

import { formatGeodeticCoordinates, i18n } from "$stores/i18n";

import gpsIcon from "$assets/svg/gps.svg?raw";

export let sns: RawSns | undefined
// TODO: export let sensor: Sensor

$: snsCoordinates = sns ? formatGeodeticCoordinates(sns.position) : ["-", "-"]

function toSnsClass(fix?: number) {
    if (!fix) {
        return "text-neutral"
    } else if (fix == 1) {
        return "text-error"
    } else if (fix == 2) {
        return "text-warning"
    } else {
        return ""
    }
}

function toSnsText(fix?: number) {
    if (!fix) {
        return $i18n.t("No GPS connected")
    } else if (fix == 1) {
        return $i18n.t("No position information")
    } else if (fix == 2) {
        return $i18n.t("2D position")
    } else if (fix == 3) {
        return $i18n.t("3D position")
    } else if (fix == 4) {
        return $i18n.t("DGPS/SBAS aided 3D position")
    } else if (fix == 5) {
        return $i18n.t("RTK float, 3D position")
    } else if (fix == 6) {
        return $i18n.t("RTK fixed, 3D position")
    } else if (fix == 7) {
        return $i18n.t("Static RTK, 3D position")
    } else if (fix == 8) {
        return $i18n.t("PPP, 3D position")
    } else {
        return $i18n.t("Unknown GPS fix")
    }
}

</script>

<div class="dropdown dropdown-hover dropdown-bottom dropdown-end">
    <div tabindex="0" role="button" class={"btn-xs fill-current " + toSnsClass(sns?.fix) }>
        { @html gpsIcon }
    </div>
    <div tabindex="0" class="dropdown-content z-[1] p-2 w-48 shadow badge-neutral rounded-md flex flex-col align-middle">
        <p class="text-center font-bold">{ toSnsText(sns?.fix) }</p>
        <div class="flex justify-between">
            <div class="text-left">{ $i18n.t("Satellites count") + ":" }</div>
            <div class="text-right">{ sns?.satellites_visible || "-" }</div>
        </div>
        <div class="flex justify-between">
            <div class="text-left">{ $i18n.t("HDOP") + ":" }</div>
            <div class="text-right">{ sns?.eph || "-" }</div>
        </div>
        <div class="flex justify-between">
            <div class="text-left">{ $i18n.t("VDOP") + ":" }</div>
            <div class="text-right">{ sns?.epv || "-" }</div>
        </div>
        <div class="flex justify-between">
            <div class="text-left">{ $i18n.t("Lat") + ":" }</div>
            <div class="text-right">{ snsCoordinates[0] }</div>
        </div>
        <div class="flex justify-between">
            <div class="text-left">{ $i18n.t("Lon") + ":" }</div>
            <div class="text-right">{ snsCoordinates[1] }</div>
        </div>
        <div class="flex justify-between">
            <div class="text-left">{ $i18n.t("Alt") + ":" }</div>
            <div class="text-right">{ sns ? sns.position.altitude + " m" : "-" }</div>
        </div>
    </div>
</div>

<!-- // TODO: charging icon when current < 0 -->
