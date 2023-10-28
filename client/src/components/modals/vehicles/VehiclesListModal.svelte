<script lang="ts">
import { clickOutside } from '$lib/common/click-outside';

import BaseModal from "$components/common/BaseModal.svelte";
import VehicleItem from "./VehicleItem.svelte";

import { VehicleType } from "$bindings/vehicles";
import { vehicleDescriptions, getNextAvailableMavlinkId } from "$stores/vehicles";
import { i18n } from "$stores/i18n";

export let editingVehicleID = ""

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
        <details id="newVehicleDropdown" class="dropdown absolute left-2 top-2" use:clickOutside={closeDropdown}>
            <summary class="btn m-1">{ $i18n.t("Add Vehicle") }</summary>
            <ul class="dropdown-content z-[3] menu p-2 shadow bg-base-300 rounded-box w-48">
                <!-- MAVLINK VEHCILE -->
                <li on:click={async () => {
                    const mavId = getNextAvailableMavlinkId()
                    if (!mavId) {
                        // TODO: warn no free id here, or move to backend
                        return;
                    }
                    const created = await vehicleDescriptions.saveVehicle({
                        name: $i18n.t("New Vehicle") + " (MAV " + mavId + ")",
                        protocol_id: { MavlinkId: { mav_id: mavId } },
                        vehicle_type: VehicleType.Auto,
                        features: []
                    });
                    if (!!created) {
                        editingVehicleID = created.id || "";
                    }
                    closeDropdown();
                    }}><a>{ $i18n.t("New MAVLink Vehicle") }</a></li>
            </ul>
        </details>
    </form>
    <h3 class="font-bold text-lg text-center mb-4">{ $i18n.t("Vehicles") }</h3>

    <!-- LIST COMPONENT -->
    <div class="space-y-2 max-scroll-area-height overflow-y-auto">
    {#each $vehicleDescriptions.values() as vehicle}
        <VehicleItem vehicle={vehicle} bind:editingVehicleID={editingVehicleID}/>
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