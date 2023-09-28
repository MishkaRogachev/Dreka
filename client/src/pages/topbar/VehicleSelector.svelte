<script lang="ts">
import Button from "$components/controls/Button.svelte"
import OverlayButton from "$components/controls/OverlayButton.svelte";
import Led from "$components/controls/Led.svelte";

import { availableVehicles, selectedVehicle, addNewVehicle } from "$stores/vehicles";

import plusIcon from "$assets/svg/plus.svg";

let overlay: any

</script>
<Button
    icon={plusIcon}
    right_cropped={true}
    flat={true}
    on:click={addNewVehicle}
/>

<OverlayButton
    bind:this={overlay}
    style="width: 216px; height: 24px"
    left_cropped={true}
    disabled={$availableVehicles.length === 0}
    text={$selectedVehicle ? $selectedVehicle.name : "No vehicle selected"}
    flat={true}
    opacity={false}>
    <div style="width:208px; max-height:256px">
        {#each $availableVehicles as vehicle}
            <Button style="width:100%" flat={true} selected={vehicle == $selectedVehicle} text={vehicle.name}
                on:click={() => { selectedVehicle.set(vehicle); overlay.close() }} />
        {/each}
    </div>
</OverlayButton>

<Led state={$selectedVehicle && $selectedVehicle.online ? "on" : "off"} />