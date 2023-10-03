<script lang="ts">
import Button from "$components/controls/Button.svelte"
import OverlayButton from "$components/controls/OverlayButton.svelte";
import Led from "$components/controls/Led.svelte";
import VehicleSelectorItem from "$pages/topbar/VehicleSelectorItem.svelte";

import { availableVehicles, selectedVehicle } from "$stores/vehicles";
import { iconFromVehicleType } from "$bindings/vehicles";

let overlay: any

</script>

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
</OverlayButton>
