<script lang="ts">
import { VehicleType } from "$bindings/vehicles";

import unknownIcon from "$assets/svg/unknown.svg"
import autoIcon from "$assets/svg/auto.svg"
import vtolIcon from "$assets/svg/vtol.svg"
import fixedWingIcon from "$assets/svg/fixed_wing.svg"
import rotaryWingIcon from "$assets/svg/rotary_wing.svg"
import copterIcon from "$assets/svg/copter.svg"
import airshipIcon from "$assets/svg/airship.svg"

export let vehicleType: VehicleType

$: iconSource = iconFromType(vehicleType)

function iconFromType(vehicleType: VehicleType) {
    switch (vehicleType) {
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
        case VehicleType.Airship:
            return airshipIcon;
    }
    return unknownIcon;
}
</script>

<img src={iconSource} class="w-8 h-8" alt="*">
