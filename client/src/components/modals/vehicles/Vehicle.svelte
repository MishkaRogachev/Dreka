<script lang="ts">
import { onMount, onDestroy, afterUpdate } from 'svelte';

import { type VehicleDescription, type VehicleStatus, is_vehicle_online } from "$bindings/vehicles";
import { vehicleStatuses, vehicleDescriptions } from "$stores/vehicles";

import { i18n } from "$stores/i18n";

export let selectedVehicleId = ""

export let vehicle: VehicleDescription
export let changed: boolean = false

let vehicleCopy: VehicleDescription = vehicle

$: status = $vehicleStatuses.get(vehicle.id || "")

afterUpdate(async () => {
    if (selectedVehicleId !== vehicle.id) {
        vehicleCopy = vehicle;
    } else {
        changed = JSON.stringify(vehicleCopy) !== JSON.stringify(vehicle);
    }
});

</script>

<div class="collapse collapse-arrow bg-base-200">
    <input type="radio" name="communication-vehicles-accordion" on:change={() => { selectedVehicleId = vehicle.id || "" }}/> 
    <div class="collapse-title flex flex-row gap-4">
        <div class="indicator w-full">
            <span class={"indicator-item badge badge-xs indicator-start indicator-middle " +
                (is_vehicle_online(status) ? "bg-success" : "bg-neutral-content")} >
            </span>
            <h1 class="font-medium ml-8 my-2">{vehicle.name}</h1>
        </div>
        <div class="join btn-sm p-0 z-[1]">
        </div>
    </div>
    <div class="collapse-content gap-8">
        <div class="grid grid-cols-2 gap-2">
            <!-- Name -->
            <h1 class="font-medium my-2 w-full">{ $i18n.t("Name") }</h1>
            <input type="text" placeholder={ $i18n.t("Enter name here") } class="input w-full"
                bind:value={vehicleCopy.name}/>

            <!-- Vehicle Type -->

            <!-- Protocol -->
            <!-- <h1 class="font-medium my-2 w-full">{ $i18n.t("Protocol ID") }</h1>
            <input type="text" placeholder={ $i18n.t("Enter protocol id here") } class="input w-full"
                bind:value={vehicleCopy.protocol_id}/> -->
        </div>

        <div class="w-full btn-sm mt-4 flex">
            <button disabled={is_vehicle_online(status)} class="btn btn-sm btn-wide btn-secondary px-1 ml-2"
                on:click={() => { vehicleDescriptions.removeVehicle(vehicle.id || "") }}>
                { $i18n.t("Remove") }
            </button>

            <div class="grow"/>

            <div class="join btn-sm p-0">
                <button disabled={!changed} class="btn btn-sm btn-wide btn-primary join-item px-1 ml-2"
                    on:click={()=> { vehicleCopy = vehicle }}>
                    { $i18n.t("Discard") }
                </button>
                <button disabled={!changed} class="btn btn-sm btn-wide btn-accent join-item px-1 ml-2"
                    on:click={ async () => { vehicle = await vehicleDescriptions.saveVehicle(vehicleCopy) || vehicle }}>
                    { $i18n.t("Save") }
                </button>
            </div>
        </div>
    </div>
</div>