<script lang="ts">
import VehicleTypeIcon from '$components/common/VehicleTypeIcon.svelte';

import type { Vehicle } from '$stores/vehicles';
import { i18n } from '$stores/i18n';

export let vehicle: Vehicle | undefined

</script>

<div class="flex gap-x-2 items-center grow">
    <VehicleTypeIcon
        vehicleType={vehicle ? vehicle.description.vehicle_type : undefined}
        color={vehicle ? vehicle.description.color : ""}/>
    <a class="grow">{vehicle ? vehicle.description.name : $i18n.t("No vehicle") }</a>
    <span class={"badge badge-xs " + (vehicle && vehicle.is_online() ? "bg-success" : "bg-neutral-content")} ></span>
</div>
