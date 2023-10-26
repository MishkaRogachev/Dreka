<!-- <script lang="ts">

import { availableVehicles, selectedVehicle } from "$stores/vehicles";
import { iconFromVehicleType } from "$bindings/vehicles";

let overlay: any

</script> -->
<!-- 
<OverlayButton
    bind:this={overlay}
    style="width: 216px; height: 24px"
    icon={$selectedVehicle ? iconFromVehicleType($selectedVehicle.vehicle_type) : ""}
    text={$selectedVehicle ? $selectedVehicle.name : "No vehicles"}
    disabled={$availableVehicles.length === 0}
    flat={true}>
    <div style="width:208px; max-height:256px">
        {#each $availableVehicles as vehicle}
            <VehicleSelectorItem vehicle={vehicle} on:activate={() => { selectedVehicle.set(vehicle); overlay.close() }} />
        {/each}
    </div>
    <Led slot="decoration" state={$selectedVehicle && $selectedVehicle.online ? "on" : "off"} />
</OverlayButton> -->


<!-- import unknownIcon from "$assets/svg/about.svg"
import autoIcon from "$assets/svg/auto.svg"
import fixedWingIcon from "$assets/svg/fixed_wing.svg"
import rotaryWingIcon from "$assets/svg/rotary_wing.svg"
import copterIcon from "$assets/svg/copter.svg"
import vtolIcon from "$assets/svg/vtol.svg"

export function iconForVehicleType(vehicle_type: VehicleType): string {
    switch (vehicle_type) {
        case VehicleType.Auto:
            return autoIcon;
        case VehicleType.FixedWing:
            return fixedWingIcon;
        case VehicleType.RotaryWing:
            return rotaryWingIcon;
        case VehicleType.Copter:
            return copterIcon;
        case VehicleType.Vtol:
            return vtolIcon;
    }
    return unknownIcon;
} -->