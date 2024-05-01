<script lang="ts">

import { i18n } from '$stores/i18n';
import { activateDialog } from '$stores/app';

import Dropdown from '$components/common/Dropdown.svelte';

import configureIcon from "$assets/svg/configure.svg?raw";
import lockIcon from "$assets/svg/lock.svg?raw";
import calibrateIcon from "$assets/svg/calibrate.svg?raw";
// import takeoffIcon from "$assets/svg/takeoff.svg?raw";
// import landIcon from "$assets/svg/land.svg?raw";
// import goAroundIcon from "$assets/svg/go_around.svg?raw";


let closeDropdown: () => void;

async function armDisarm() {
    activateDialog((await import('$components/dialogs/ArmDisarm.svelte')).default);
    closeDropdown();
}

async function calibrate() {
    activateDialog((await import('$components/dialogs/Calibrate.svelte')).default);
    closeDropdown();
}

function takeoff() {
    console.log("Takeoff");
    closeDropdown();
}

function land() {
    console.log("Land");
    closeDropdown();
}

function goAround() {
    console.log("Go Around");
    closeDropdown();
}

</script>

<Dropdown bind:closeDropdown={closeDropdown} tip={ $i18n.t("Command vehicle") }>
    <span slot="summary">{ @html configureIcon }</span>
    <ul slot="details" class="menu p-0">
        <li class="flex" on:click={armDisarm}>
            <div class="flex gap-x-2 items-center grow">
                { @html lockIcon }
                <a href={null} class="grow ">{ $i18n.t("ARM/DISARM") }</a>
            </div>
        </li>
        <li class="flex" on:click={calibrate}>
            <div class="flex gap-x-2 items-center grow">
                { @html calibrateIcon }
                <a href={null} class="grow ">{ $i18n.t("CALIBRATE") }</a>
            </div>
        </li>
        <!-- <li class="flex" on:click={takeoff}>
            <div class="flex gap-x-2 items-center grow">
                { @html takeoffIcon }
                <a href={null} class="grow ">{ $i18n.t("TAKEOFF") }</a>
            </div>
        </li>
        <li class="flex" on:click={land}>
            <div class="flex gap-x-2 items-center grow">
                { @html landIcon }
                <a href={null} class="grow ">{ $i18n.t("LAND") }</a>
            </div>
        </li>
        <li class="flex" on:click={goAround}>
            <div class="flex gap-x-2 items-center grow">
                { @html goAroundIcon }
                <a href={null} class="grow ">{ $i18n.t("GO AROUND") }</a>
            </div>
        </li> -->
    </ul>
</Dropdown>
