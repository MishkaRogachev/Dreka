<script lang="ts">

import { i18n } from "$stores/i18n";
import { missions, selectedVehicleMission } from "$stores/mission";
import { selectedVehicle } from "$stores/vehicles";

import missionIcon from "$assets/svg/mission.svg?raw";
import downloadIcon from "$assets/svg/download.svg?raw";
import uploadIcon from "$assets/svg/upload.svg?raw";
import removeIcon from "$assets/svg/remove.svg?raw";
import plusIcon from "$assets/svg/plus.svg?raw";

$: missionId = $selectedVehicleMission?.id;

function downloadMission() {
    if (missionId) {
        missions.download(missionId);
    }
}

function uploadMission() {
    if (missionId) {
        missions.upload(missionId);
    }
}

function clearMission() {
    if (missionId) {
        missions.clear(missionId);
    }
}

function createMission() {
    if ($selectedVehicle) {
        missions.createVehicleMission($selectedVehicle.description.id);
    }
}

</script>

<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Mission") }>
    <div class="dropdown dropdown-start">
        <div tabindex="0" role="button" class="select select-ghost select-sm m-1 items-center">
            { @html missionIcon }
        </div>
        <ul tabindex="0" class="w-48 dropdown-content menu z-[1] p-0 shadow bg-base-300 rounded-md my-0">
        {#if !!missionId}
            <li class="flex" on:click={downloadMission}>
                <div class="flex gap-x-2 items-center grow">
                    { @html downloadIcon }
                    <a class="grow">{ $i18n.t("Download mission") }</a>
                </div>
            </li>
            <li class="flex" on:click={uploadMission}>
                <div class="flex gap-x-2 items-center grow">
                    { @html uploadIcon }
                    <a class="grow">{ $i18n.t("Upload mission") }</a>
                </div>
            </li>
            <li class="flex" on:click={clearMission}>
                <div class="flex gap-x-2 items-center grow">
                    { @html removeIcon }
                    <a class="grow">{ $i18n.t("Clear mission") }</a>
                </div>
            </li>
        {:else if !!$selectedVehicle}
        <li class="flex" on:click={createMission}>
            <div class="flex gap-x-2 items-center grow">
                { @html plusIcon }
                <a class="grow">{ $i18n.t("Create new mission") }</a>
            </div>
        </li>
        {/if}
        </ul>
    </div>
</div>
