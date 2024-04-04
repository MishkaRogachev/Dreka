<script lang="ts">
import { clickOutside } from '$lib/common/click-outside';

import VehicleTypeIcon from '$components/common/VehicleTypeIcon.svelte';

import { selectedVehicleID, selectedVehicle, vehicles } from "$stores/vehicles";
import { i18n } from '$stores/i18n';

function closeDropdown() {
    document.getElementById("vehicleSelectorDropdown")?.removeAttribute("open");
}

</script>

<div id="vehicleSelectorDropdown" class="dropdown dropdown-end" use:clickOutside={closeDropdown}>
    <div tabindex="0" class="select select-ghost select-sm m-1 gap-x-2 items-center btn-wide">
        <VehicleTypeIcon vehicleType={$selectedVehicle?.description.vehicle_type} color={$selectedVehicle?.description.color || ""}/>
        <a class="grow">{$selectedVehicle ? $selectedVehicle?.description.name || "" : $i18n.t("No vehicle") }</a>
    </div>
    <ul tabindex="0" class="dropdown-content menu z-[1] p-0 shadow bg-base-300 rounded-md my-0">
    {#each $vehicles.values() as vehicle}
        <li class="btn-wide flex" on:click = {() => { selectedVehicleID.set(vehicle.description.id || ""); closeDropdown(); }}>
            <div class="flex gap-x-2 items-center grow">
                <VehicleTypeIcon vehicleType={vehicle.description.vehicle_type} color={vehicle.description.color}/>
                <a class={"grow " + (vehicle.description.id === $selectedVehicleID ? "text-white" : "")}>
                    {vehicle.description.name}
                </a>
                <span class={"badge badge-xs " + (vehicle.is_online ? "bg-success" : "bg-neutral-content")} ></span>
            </div>
        </li>
    {/each}
    </ul>
</div>
