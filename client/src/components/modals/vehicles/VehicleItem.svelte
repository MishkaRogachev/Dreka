<script lang="ts">
import { afterUpdate } from 'svelte';

import VehicleTypeIcon from '$components/common/VehicleTypeIcon.svelte';
import MavlinkIdEdit from '$components/modals/vehicles/MavlinkIdEdit.svelte';

import { type VehicleDescription } from "$bindings/vehicles";
import { Vehicle, vehicles, vehicleTypes, selectedVecicleID } from "$stores/vehicles";

import { i18n } from "$stores/i18n";

export let editingVehicleID = ""

export let vehicle: Vehicle
export let changed: boolean = false

let descriptionCopy: VehicleDescription = vehicle.description

afterUpdate(async () => {
    if (editingVehicleID !== vehicle.description.id) {
        descriptionCopy = vehicle.description;
    } else {
        changed = JSON.stringify(descriptionCopy) !== JSON.stringify(vehicle.description);
    }
});

</script>

<div class="collapse collapse-arrow bg-base-200">
    <input type="radio" checked={ editingVehicleID === vehicle.description.id } name="communication-vehicles-accordion"
        on:change={() => { editingVehicleID = vehicle.description.id || "" }}/>
    <div class="collapse-title flex flex-row gap-4 w-full content-center">
        <!-- VEHICLE INDICATOR -->
        <div class="indicator">
            <span class={"indicator-item badge badge-xs " +
                (vehicle.is_online() ? "bg-success" : "bg-neutral-content")} >
            </span>
            <VehicleTypeIcon vehicleType={vehicle.description.vehicle_type} />
        </div>

        <!-- VEHICLE TITLE -->
        <h1 class="font-medium ml-2 my-2">{vehicle.description.name}</h1>

        <div class="grow"/>

        <!-- SELECT VECICLE -->
        {#if $selectedVecicleID === vehicle.description.id}
            <button class="btn btn-sm z-[1] btn-outline btn-disabled">{ $i18n.t("Selected") }</button>
        {/if}

        <div class="join btn-sm p-0 z-[1]">
        </div>
    </div>
    <div class="collapse-content gap-8">
        <div class="grid grid-cols-2 gap-2">
            <!-- NAME -->
            <h1 class="font-medium my-2 w-full">{ $i18n.t("Name") }</h1>
            <input type="text" placeholder={ $i18n.t("Enter name here") } class="input w-full" disabled={vehicle.is_online()}
                bind:value={descriptionCopy.name}/>

            <!-- VEHICLE TYPE -->
            <h1 class="font-medium my-2 w-full">{ $i18n.t("Type") }</h1>
            <select class="select w-full" bind:value={descriptionCopy.vehicle_type} disabled={vehicle.is_online()}>
                {#each vehicleTypes as type, i}
                <option value={type} disabled={i === 0}> { $i18n.t(type) }</option>
                {/each}
            </select>

            <!-- TODO: VEHICLE FEATURES -->

            <!-- PROTOCOL ID -->
            {#if descriptionCopy.protocol_id.MavlinkId}
                <MavlinkIdEdit bind:protocol_id={descriptionCopy.protocol_id.MavlinkId} disabled={vehicle.is_online()}/>
            {/if}
        </div>

        <div class="w-full btn-sm mt-4 flex">
            <button disabled={vehicle.is_online()} class="btn btn-sm btn-wide btn-outline btn-secondary px-1 ml-2"
                on:click={() => { vehicles.removeVehicle(vehicle.description.id || "") }}>
                { $i18n.t("Remove") }
            </button>

            <div class="grow"/>

            <div class="join btn-sm p-0">
                <button disabled={!changed} class="btn btn-sm btn-wide btn-primary join-item px-1 ml-2"
                    on:click={()=> { descriptionCopy = vehicle.description }}>
                    { $i18n.t("Discard") }
                </button>
                <button disabled={!changed} class="btn btn-sm btn-wide btn-accent join-item px-1 ml-2"
                    on:click={ async () => { vehicle = await vehicles.saveVehicle(descriptionCopy) || vehicle }}>
                    { $i18n.t("Save") }
                </button>
            </div>
        </div>
    </div>
</div>