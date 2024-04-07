<script lang="ts">
import type { Vehicle } from "$stores/vehicles";
import type { Mission, MissionUpdateState } from "$bindings/mission";

import { i18n } from "$stores/i18n";
import { missions, selectedVehicleMission } from "$stores/mission";

import missionIcon from "$assets/svg/mission.svg?raw";
import downloadIcon from "$assets/svg/download.svg?raw";
import uploadIcon from "$assets/svg/upload.svg?raw";
import removeIcon from "$assets/svg/remove.svg?raw";
import cancelIcon from "$assets/svg/cancel.svg?raw";
import plusIcon from "$assets/svg/plus.svg?raw";

export let vehicle: Vehicle;

let progressValue = 0;
let progressMax = 0;
let progressClass = "";
let missionStateText = "";

$: missionId = $selectedVehicleMission?.id;
$: updateFromMissionState($selectedVehicleMission);

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

function cancelState() {
    if (missionId) {
        missions.cancelState(missionId);
    }
}

function createMission() {
    missions.createVehicleMission(vehicle.description.id);
}

function updateFromMissionState(mission: Mission | undefined) {
    if (!mission) {
        missionStateText = $i18n.t("No mission");
        progressValue = 0;
        progressMax = 1;
        progressClass = "";
    } else if (mission.status.state.Actual) {
        missionStateText = $i18n.t("Actual mission") + ": " + mission.status.state.Actual.total + " " + $i18n.t("items");
        progressValue = 1;
        progressMax = 1;
        progressClass = "progress-success";
    } else if (mission.status.state.NotActual) {
        missionStateText = $i18n.t("Mission not actual");
        progressValue = 1;
        progressMax = 1;
        progressClass = "progress-warning";
    } else if (mission.status.state.PrepareDownload) {
        missionStateText = $i18n.t("Preparing download..");
        progressValue = 1;
        progressMax = 100;
        progressClass = "progress-primary";
    } else if (mission.status.state.Download) {
        missionStateText = $i18n.t("Downloading") + ": " + mission.status.state.Download.progress +
            "/" + mission.status.state.Download.total + " " + $i18n.t("items");
        progressValue = mission.status.state.Download.progress;
        progressMax = mission.status.state.Download.total;
        progressClass = "progress-primary";
    } else if (mission.status.state.PrepareUpload) {
        missionStateText = $i18n.t("Preparing upload..");
        progressValue = 1;
        progressMax = mission.status.state.PrepareUpload.total;
        progressClass = "progress-accent";
    } else if (mission.status.state.Upload) {
        missionStateText = $i18n.t("Uploading") + ": " + mission.status.state.Upload.progress +
            "/" + mission.status.state.Upload.total + " " + $i18n.t("items");
        progressValue = mission.status.state.Upload.progress;
        progressMax = mission.status.state.Upload.total;
        progressClass = "progress-accent";

    } else if (mission.status.state.Clearing) {
        missionStateText = $i18n.t("Clearing mission");
        progressValue = 1;
        progressMax = 1;
        progressClass = "progress-secondary";
    } else {
        missionStateText = $i18n.t("Unkown mission state");
        progressValue = 1;
        progressMax = 1;
        progressClass = "progress-error";
    }
}

</script>

<div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Mission") }>
    <div class="dropdown dropdown-start">
        <div tabindex="0" role="button" class="select select-ghost select-sm m-1 items-center">
            { @html missionIcon }
        </div>
        <div tabindex="0" class="w-64 dropdown-content z-[1] p-2 shadow bg-base-300 rounded-md my-0">
            <div class="font-bold">{ missionStateText }</div>
            <progress class={"progress grow " + progressClass}
                value={progressValue} max={progressMax}></progress>
            <ul class="menu p-0">
            {#if $selectedVehicleMission &&
                ($selectedVehicleMission.status.state.Actual || $selectedVehicleMission.status.state.NotActual)}
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
            {/if}
            {#if $selectedVehicleMission &&
                ($selectedVehicleMission.status.state.PrepareUpload || $selectedVehicleMission.status.state.Upload ||
                $selectedVehicleMission.status.state.Download || $selectedVehicleMission.status.state.PrepareDownload ||
                $selectedVehicleMission.status.state.Clearing)}
            <li class="flex" on:click={cancelState}>
                <div class="flex gap-x-2 items-center grow">
                    { @html cancelIcon }
                    <a class="grow">{ $i18n.t("Cancel operation") }</a>
                </div>
            </li>
            {/if}
            {#if !$selectedVehicleMission}
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
</div>
