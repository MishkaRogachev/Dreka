<script lang="ts">
import { VehicleType } from "$bindings/vehicles";

import unknownIcon from "$assets/svg/unknown.svg?raw"
import autoIcon from "$assets/svg/auto.svg?raw"
import vtolIcon from "$assets/svg/vtol.svg?raw"
import fixedWingIcon from "$assets/svg/fixed_wing.svg?raw"
import rotaryWingIcon from "$assets/svg/rotary_wing.svg?raw"
import copterIcon from "$assets/svg/copter.svg?raw"
import airshipIcon from "$assets/svg/airship.svg?raw"

export let vehicleType: VehicleType | undefined

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

{@html vehicleType ? iconFromType(vehicleType) : ""}
