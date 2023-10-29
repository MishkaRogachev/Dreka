<script lang="ts">
import { clickOutside } from '$lib/common/click-outside';

import VehicleSelectorItem from "$components/topbar/VehicleSelectorItem.svelte";

import { selectedVehicleID, selectedVehicle, vehicles } from "$stores/vehicles";

function closeDropdown() {
    document.getElementById("vehicleSelectorDropdown")?.removeAttribute("open");
}

</script>

<details id="vehicleSelectorDropdown" class="dropdown" use:clickOutside={closeDropdown}>
    <summary class="btn btn-ghost btn-sm btn-wide rounded-btn m-1">
        <VehicleSelectorItem vehicle={$selectedVehicle} />
    </summary>
    <ul class="dropdown-content menu z-[1] p-0 shadow bg-base-300 rounded-md my-0">
    {#each $vehicles.values() as vehicle}
        <li class={"btn-wide flex " + (vehicle.description.id === $selectedVehicleID ? "disabled" : "")}
            on:click = {() => {
                selectedVehicleID.set(vehicle.description.id || "");
                closeDropdown();
            }}>
            <VehicleSelectorItem vehicle={vehicle} />
        </li>
    {/each}
    </ul>
</details>
