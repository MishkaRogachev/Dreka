<script lang="ts">
import { createEventDispatcher } from "svelte"

import Button from "$components/controls/Button.svelte";
import Led from "$components/controls/Led.svelte";

import { selectedVehicle } from "$stores/vehicles";
import { type VehicleDescription, VehicleType } from "$bindings/vehicles";

import fixedWingIcon from "$assets/svg/fixed_wing.svg"
import rotaryWingIcon from "$assets/svg/rotary_wing.svg"
import copterIcon from "$assets/svg/copter.svg"
import vtolIcon from "$assets/svg/vtol.svg"

export let vehicle: VehicleDescription

const dispatch = createEventDispatcher()

function activate() { dispatch('activate', {}); }

function iconFromVehicleType(): string {
    if (!vehicle)
        return ""

    switch (vehicle.vehicle_type) {
        case VehicleType.FixedWing:
            return fixedWingIcon;
        case VehicleType.RotaryWing:
            return rotaryWingIcon;
        case VehicleType.Copter:
            return copterIcon;
        case VehicleType.Vtol:
            return vtolIcon;
    }
    return ""
}
</script>

<Button
    style="width:100%"
    flat={true}
    selected={!!$selectedVehicle && vehicle.name == $selectedVehicle?.name}
    text={vehicle.name}
    icon={iconFromVehicleType()}
    on:click={activate}
>
<!-- <Led state={vehicle.online ? "on" : "off"} style="position: flex"/> -->
</Button>