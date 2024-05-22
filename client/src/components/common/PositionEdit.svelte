<script lang="ts">
import { GeodeticFrame, type Geodetic } from "$bindings/spatial";

import { i18n } from "$stores/i18n";

export let disabled: boolean = false;
export let position: Geodetic;
// TODO: dms switching
// TODO: crop values to step
</script>

<h1 class="font-medium my-2 w-full">{ $i18n.t("Latitude") + ", " + "\u00B0" }</h1>
<input disabled={disabled} type="number" step=".0000001" min="-90" max="90"
    placeholder={ $i18n.t("Zero") } class="input input-sm w-full"
    bind:value={position.latitude}/>
<h1 class="font-medium my-2 w-full">{ $i18n.t("Longitude") + ", " + "\u00B0" }</h1>
<input disabled={disabled} type="number" step=".0000001" min="-180" max="180"
    placeholder={ $i18n.t("Zero") } class="input input-sm w-full"
    bind:value={position.longitude}/>
<h1 class="font-medium my-2 w-full">{ $i18n.t("Altitude") + ", " + $i18n.t("m") }</h1>
<input disabled={disabled} type="number" step=".01" min="-500" max="50000"
    placeholder={ $i18n.t("Zero") } class="input input-sm w-full"
    bind:value={position.altitude}/>
<h1 class="font-medium my-2 w-full">{ $i18n.t("Geodetic frame") }</h1>
<select disabled={disabled} class="select select-sm" bind:value={position.frame}>
    <option value={GeodeticFrame.Wgs84AboveSeaLevel}>{ $i18n.t("WGS84 AMSL") }</option>
    <option value={GeodeticFrame.Wgs84RelativeHome}>{ $i18n.t("WGS84 Rel. Home") }</option>
    <!-- TODO: terrain altitude -->
    <!-- <option value={GeodeticFrame.Wgs84AboveTerrain}>{ $i18n.t("WGS84 Rel. Terrain") }</option> -->
    </select>