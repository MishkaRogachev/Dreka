<script lang="ts">
import BaseModal from "$components/common/BaseModal.svelte";
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

</script>

<style>
.max-scroll-area-height {
    max-height: 70vh;
}
</style>

<BaseModal id="vehicles_modal">
    <form method="dialog">
        <!-- CLOSE -->
        <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
        <!-- ADD NEW -->
        <details id="newVehicleDropdown" class="dropdown absolute left-2 top-2">
            <summary class="btn m-1">{ $i18n.t("Add Vehicle") }</summary>
            <ul class="dropdown-content z-[3] menu p-2 shadow bg-base-300 rounded-box w-48">
                {#each vehiclesForCreation as vehicle}
                    <li on:click={async () => {
                        const created = await vehicleDescriptions.saveVehicle(vehicle);
                        if (!!created) {
                            selectedVehicleId = created.id || "";
                        }
                        closeDropdown();
                        }}><a>{ vehicle.name }</a></li>
                {/each}
            </ul>
        </details>
    </form>
    <h3 class="font-bold text-lg text-center mb-4">{ $i18n.t("Vehicles") }</h3>

    <!-- LIST COMPONENT -->
    <div class="space-y-2 max-scroll-area-height overflow-y-auto">
    {#each $vehicleDescriptions.values() as vehicle}
        <Vehicle vehicle={vehicle} bind:selectedVehicleId={selectedVehicleId}/>
    {/each}
    </div>

    <!-- FILLER -->
    <div class="flex flex-col grow text-center">
    {#if $vehicleDescriptions.size === 0}
        <a class="grow">{ $i18n.t("No vehicles available") }</a>
    {:else}
        <div class="grow"/>
    {/if}
    </div>

    <div class="divider"></div>

    <!-- ADD VEHCILES ON HEARTBEAT TODO: backend -->
    <div class="form-control grow-0">
        <label class="label cursor-pointer">
            <span class="label-text">{ $i18n.t("Add vehicles on heartbeat") }</span>
            <input type="checkbox" class="checkbox" />
        </label>
    </div>
</BaseModal>