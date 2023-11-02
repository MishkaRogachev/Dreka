<script lang="ts">
import { VehicleState } from "$bindings/vehicles";

import { i18n } from "$stores/i18n";
import { selectedVehicle } from "$stores/vehicles";

$: is_online = $selectedVehicle?.is_online()
$: vehicleState = $selectedVehicle?.status?.state || VehicleState.Unknown

</script>

<!-- Boot = "Boot",
Calibrating = "Calibrating",
Standby = "Standby",
Active = "Active",
Critical = "Critical",
Emergency = "Emergency",
PowerOff = "PowerOff",
FlightTermination = "FlightTermination", -->
<div class="flex items-center gap-x-2">
    { $i18n.t(vehicleState) }
    <span class={"badge badge-xs " + (is_online ? "bg-success" : "bg-neutral-content")} ></span>
</div>