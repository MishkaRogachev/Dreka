<script lang="ts">
import { VehicleState } from "$bindings/vehicles";

import { i18n } from "$stores/i18n";
import { selectedVehicle } from "$stores/vehicles";

$: is_online = $selectedVehicle?.is_online()
$: vehicleState = $selectedVehicle?.status?.state || VehicleState.Unknown

</script>

<div class="flex items-center gap-x-2 font-bold">
    <a>{ $i18n.t(vehicleState) }</a>
    <span class={"badge badge-xs " + (is_online ? "bg-success" : "bg-neutral-content")} ></span>
    <!-- TODO: flight time -->
</div>