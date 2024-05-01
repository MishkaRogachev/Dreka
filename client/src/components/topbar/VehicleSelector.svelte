<script lang="ts">
import { EntityColor } from "$bindings/colors";
import { VehicleType } from "$bindings/vehicles";

import { i18n } from '$stores/i18n';
import { selectedVehicleId, selectedVehicle, vehicles } from "$stores/vehicles";

import VehicleTypeIcon from '$components/common/VehicleTypeIcon.svelte';
import Dropdown from '$components/common/Dropdown.svelte';

$: availableVehicles = [...$vehicles.values()];

let closeDropdown: () => void;

</script>

<Dropdown bind:closeDropdown={closeDropdown} tip={ $i18n.t("Select vehicle") } empty={availableVehicles.length === 0}>
    <div slot="summary" class="flex gap-x-2 items-center text-sm font-mono text-nowrap">
        <VehicleTypeIcon
            vehicleType={$selectedVehicle?.description.vehicle_type || VehicleType.Unknown}
            color={$selectedVehicle?.description.color || EntityColor.Slate} />
        <span>{$selectedVehicle ? $selectedVehicle?.description.name || "" : $i18n.t("No vehicle") }</span>
    </div>
    <ul slot="details" class="menu p-0">
    {#each availableVehicles as vehicle}
        <li on:click={() => { selectedVehicleId.set(vehicle.description.id || ""); closeDropdown(); }}>
            <div class="flex gap-x-2 items-center grow font-mono text-nowrap">
                <VehicleTypeIcon vehicleType={vehicle.description.vehicle_type} color={vehicle.description.color}/>
                <a href={null} class={"grow " + (vehicle.description.id === $selectedVehicleId ? "font-black" : "font-normal")}>
                    {vehicle.description.name}
                </a>
                <span class={"badge badge-xs " + (vehicle.is_online ? "bg-success" : "bg-neutral-content")} ></span>
            </div>
        </li>
    {/each}
    </ul>
</Dropdown>
