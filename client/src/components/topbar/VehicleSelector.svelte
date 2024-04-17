<script lang="ts">
import { EntityColor } from "$bindings/colors";
import { VehicleType } from "$bindings/vehicles";

import { i18n } from '$stores/i18n';
import { selectedVehicleID, selectedVehicle, vehicles } from "$stores/vehicles";

import VehicleTypeIcon from '$components/common/VehicleTypeIcon.svelte';

</script>

<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Select vehicle") }>
    <div class="dropdown dropdown-end">
        <div tabindex="0" class="select select-ghost select-sm m-1 gap-x-2 items-center w-64">
            <VehicleTypeIcon
                vehicleType={$selectedVehicle?.description.vehicle_type || VehicleType.Unknown}
                color={$selectedVehicle?.description.color || EntityColor.Slate}
            />
            <a class="grow">{$selectedVehicle ? $selectedVehicle?.description.name || "" : $i18n.t("No vehicle") }</a>
        </div>
        <ul tabindex="0" class="dropdown-content menu z-[1] p-0 shadow bg-base-300 rounded-md my-0">
        {#each $vehicles.values() as vehicle}
            <li class="w-64 flex" on:click = {() => { selectedVehicleID.set(vehicle.description.id || ""); }}>
                <div class="flex gap-x-2 items-center grow">
                    <VehicleTypeIcon vehicleType={vehicle.description.vehicle_type} color={vehicle.description.color}/>
                    <a class={"grow " + (vehicle.description.id === $selectedVehicleID ? "font-black" : "font-normal")}>
                        {vehicle.description.name}
                    </a>
                    <span class={"badge badge-xs " + (vehicle.is_online ? "bg-success" : "bg-neutral-content")} ></span>
                </div>
            </li>
        {/each}
        </ul>
    </div>
</div>