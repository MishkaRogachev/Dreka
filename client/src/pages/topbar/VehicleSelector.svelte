<script lang="ts">
import Button from "$components/controls/Button.svelte"
import OverlayButton from "$components/controls/OverlayButton.svelte";
import Led from "$components/controls/Led.svelte";
import VehicleSelectorItem from "$pages/topbar/VehicleSelectorItem.svelte";

import { isServerOnline } from '$stores/app';
import { availableVehicles, selectedVehicle, addNewVehicle } from "$stores/vehicles";

import configureIcon from "$assets/svg/configure.svg"

let overlay: any

</script>

<Button
    icon={configureIcon}
    right_cropped={true}
    flat={true}
    disabled={!$selectedVehicle}
/>
<OverlayButton
    bind:this={overlay}
    style="width: 216px; height: 24px"
    left_cropped={true}
    text={$selectedVehicle ? $selectedVehicle.name : "No vehicles"}
    flat={true}>
    <div style="width:208px; max-height:256px">
        {#each $availableVehicles as vehicle}
            <VehicleSelectorItem vehicle={vehicle} on:activate={() => { selectedVehicle.set(vehicle); overlay.close() }} />
        {/each}
        <Button
            style="width:100%"
            text="Add new vehicle"
            disabled={!$isServerOnline}
        on:click={() => { addNewVehicle(); overlay.close() }}
        />
    </div>
</OverlayButton>

<Led state={$selectedVehicle && $selectedVehicle.online ? "on" : "off"} />