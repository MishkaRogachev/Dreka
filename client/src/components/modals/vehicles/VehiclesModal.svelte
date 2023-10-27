<script lang="ts">
import Vehicle from "./Vehicle.svelte";

import { type VehicleDescription, VehicleType } from "$bindings/vehicles";
import { vehicleDescriptions } from "$stores/vehicles";
import { i18n } from "$stores/i18n";

export let selectedVehicleId = ""

const vehiclesForCreation: Array<VehicleDescription> = [{
        name: $i18n.t("MAVLink Copter"),
        protocol_id: { MavlinkId: { mav_id: 1 } },
        vehicle_type: VehicleType.Copter,
        features: []
    }, {
        name: $i18n.t("MAVLink Fixed Wing"),
        protocol_id: { MavlinkId: { mav_id: 1 } },
        vehicle_type: VehicleType.FixedWing,
        features: []
    }, {
        name: $i18n.t("MAVLink VTOL"),
        protocol_id: { MavlinkId: { mav_id: 1 } },
        vehicle_type: VehicleType.Vtol,
        features: []
    }, {
        name: $i18n.t("MAVLink Rotary Wing"),
        protocol_id: { MavlinkId: { mav_id: 1 } },
        vehicle_type: VehicleType.RotaryWing,
        features: []
    }
]

function closeDropdown() {
    document.getElementById("newVehicleDropdown")?.removeAttribute("open");
}

// TODO: common modal
</script>

<style>
.max-scroll-area-height {
    max-height: 70vh;
}
</style>

<dialog id="vehicles_modal" class="modal">
    <div class="modal-box w-11/12 max-w-5xl container overflow-hidden">
        <form method="dialog">
            <!-- CLOSE -->
            <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
            <!-- ADD NEW -->
            <details id="newVehicleDropdown" class="dropdown absolute left-2 top-2">
                <summary class="btn m-1">{ $i18n.t("Add Vehicle") }</summary>
                <ul class="dropdown-content z-[3] menu p-2 shadow bg-base-300 rounded-box w-48">
                    {#each vehiclesForCreation as vehicle}
                        <li on:click={() => { vehicleDescriptions.saveVehicle(vehicle); closeDropdown(); }}><a>{ vehicle.name }</a></li>
                    {/each}
                </ul>
            </details>
        </form>
        <h3 class="font-bold text-lg text-center mb-4">{ $i18n.t("Vehicles") }</h3>

        <div class="grid gap-y-2 my-4 max-scroll-area-height overflow-y-auto text-center">
        {#each $vehicleDescriptions.values() as vehicle}
            <Vehicle vehicle={vehicle} bind:selectedVehicleId={selectedVehicleId}/>
        {/each}
        {#if $vehicleDescriptions.size === 0}
        <a>{ $i18n.t("No vehicles available") }</a>
        {/if}
        </div>
    </div>
</dialog>