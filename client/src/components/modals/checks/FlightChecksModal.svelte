<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import BaseModal from "$components/common/BaseModal.svelte";
import SensorHealth from '$components/modals/checks/SensorHealth.svelte';

import { i18n } from "$stores/i18n";
import { selectedVehicle, safetyCheck } from "$stores/vehicles";
import { selectedVehicleTelemetry } from "$stores/telemetry";

$: armed = $selectedVehicle?.status?.armed || false
$: readyToArm = $selectedVehicleTelemetry.sensors.arm_ready

$: sensors = $selectedVehicleTelemetry.sensors.sensors

let armPressed: boolean = false
let armProgress: number = 0
let interval: any;

function commandVehicleArm(arm: boolean) {
    console.log("---> TODO: ARM/DISARM:", arm)
}

onMount(async () => {
    // Update arm progress every 200ms
    interval = setInterval(() => {
        if (armPressed) {
            armProgress += 10;
            if (armProgress > 100) {
                armPressed = false;
                armProgress = 0;
                commandVehicleArm(!armed);
            }
        } else {
            armProgress = 0;
        }
    }, 100);
})

onDestroy(async () => { clearInterval(interval); })

</script>

<style>
.max-scroll-area-height {
    max-height: 30vh;
}
</style>

<BaseModal id="flight_checks_modal">
    <form method="dialog">
        <!-- CLOSE -->
        <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
    </form>
    <h3 class="font-bold text-lg text-center mb-4">{ $i18n.t("Flight Checks") }</h3>

    <!-- CHECKS LIST COMPONENT -->
    <div class="grid grid-cols-2 gap-4 max-scroll-area-height overflow-y-auto">
        {#each sensors as sensor}
        <SensorHealth sensor={sensor}/>
        {/each}
    </div>

    <!-- FILLER -->
    <div class="flex flex-col grow text-center">
    {#if sensors.length === 0}
        <a class="grow">{ $i18n.t("No sensor available") }</a>
    {:else}
        <div class="grow"/>
    {/if}
    </div>

    <div class="divider">
        <label class="label cursor-pointer gap-x-2">
            <span class="label-text">{ $i18n.t("DANGER ZONE") }</span> 
            <input type="checkbox" class="checkbox" bind:checked={$safetyCheck}/>
        </label>
    </div>

    <!-- ARM/DISARM -->
    <div class="form-control grow-0">
        <button class={"btn " + (armed ? "btn-error" : "btn-secondary") } disabled={!$safetyCheck} 
            on:mousedown={() => armPressed = true} on:mouseup={() => armPressed = false} on:mouseleave={() => armPressed = false}>
            <div class={armPressed ? "radial-progress" : ""} style="--value:{armProgress}; --size:2rem;" role="progressbar" />
            { armed ? $i18n.t("DISARM VEHICLE (Long press)") : readyToArm ? $i18n.t("ARM VEHICLE (Long press)") : $i18n.t("NOT READY (Long press to arm)") }
        </button>
    </div>
</BaseModal>
