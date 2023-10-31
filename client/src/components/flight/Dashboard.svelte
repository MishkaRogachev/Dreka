<script lang="ts">
import Ai from "$components/indicators/AI.svelte";
import Parameter from "$components/indicators/Parameter.svelte";

import { selectedVehicle } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";
import { i18n } from '$stores/i18n';

$: telemetry = $selectedVehicleTelemetry
$: online = $selectedVehicle?.is_online()

</script>

<div id="dashboard" class="absolute top-10 right-2 bg-base-200 p-2 rounded-md shadow-lg">
<!-- FLIFGHT DATA DISPLAY -->
<div class = "grid grid-cols-4 gap-2 text-center">
    <Parameter name={ $i18n.t("GS") } value={telemetry.sns.ground_speed}/>
    <div class="row-span-3 col-span-2">
    <Ai online={online} pitch={telemetry.flight.pitch} roll={telemetry.flight.roll}/>
    </div>
    <Parameter name={ $i18n.t("ASNS") } value={telemetry.sns.position.altitude}/>
    <Parameter name={ $i18n.t("IAS") } value={telemetry.flight.indicated_airspeed}/>
    <Parameter name={ $i18n.t("AMSL") } value={telemetry.flight.altitude_amsl}/>
    <a class="text-sm">{$i18n.t("m/s")}</a>
    <a class="text-sm">{$i18n.t("m")}</a>
</div>
</div>