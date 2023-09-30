<script lang="ts">
import { createEventDispatcher } from "svelte"

import Button from "$components/controls/Button.svelte";
import Led from "$components/controls/Led.svelte";

import { selectedVehicle } from "$stores/vehicles";
import { type VehicleDescription, iconFromVehicleType } from "$bindings/vehicles";

export let vehicle: VehicleDescription

const dispatch = createEventDispatcher()

function activate() { dispatch('activate', {}); }

</script>

<Button
    style="width:100%"
    flat={true}
    selected={!!$selectedVehicle && vehicle.name == $selectedVehicle?.name}
    text={vehicle.name}
    icon={iconFromVehicleType(vehicle.vehicle_type)}
    on:click={activate}
>
<Led slot="decoration" state={vehicle.online ? "on" : "off"} style="float: right;"/>
</Button>
