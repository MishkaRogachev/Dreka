<script lang="ts">
import { VehicleType } from "$bindings/vehicles";
import { toColorCode } from '$bindings/colors';

import autoIcon from "$assets/svg/auto.svg?raw"
import vtolIcon from "$assets/svg/vtol.svg?raw"
import fixedWingIcon from "$assets/svg/fixed_wing.svg?raw"
import rotaryWingIcon from "$assets/svg/rotary_wing.svg?raw"
import copterIcon from "$assets/svg/copter.svg?raw"
import airshipIcon from "$assets/svg/airship.svg?raw"

export let vehicleType: VehicleType | undefined
export let color: string

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
    return "";
}
</script>

<div style="color: {toColorCode(color)}">
    {@html vehicleType ? iconFromType(vehicleType) : ""}
</div>
