<script lang="ts">
import { Theme, theme, scale, scales } from '$stores/app';
import { locale, locales } from '$stores/i18n';

import { i18n } from '$stores/i18n';

import burgerIcon from "$assets/svg/burger.svg?raw";
import commIcon from "$assets/svg/comm.svg?raw";
import fleetIcon from "$assets/svg/fleet.svg?raw";
import aboutIcon from "$assets/svg/about.svg?raw";

function openModal(id: string) {
    // @ts-ignore
    document.getElementById(id)?.showModal();
}

</script>

<div id="menuDropdown" class="dropdown">
    <div class="tooltip tooltip-bottom" data-tip={ $i18n.t("Menu") }>
        <div tabindex="0" class="btn btn-ghost btn-sm m-1">{@html burgerIcon}</div>
    </div>
    <ul tabindex="0" class="dropdown-content menu z-[1] p-2 shadow bg-base-300 rounded-box my-0">
        <h4>{ $i18n.t("Configure") }</h4>
        <li class="btn-wide"><a href={null} on:click={() => { openModal("communication_modal"); }}>
            {@html commIcon} { $i18n.t("Communication") }
        </a></li>
        <li class="btn-wide"><a href={null} on:click={() => { openModal("vehicles_modal"); }}>
            {@html fleetIcon} { $i18n.t("Vehicles") }
        </a></li>
        <h4>{ $i18n.t("Misc") }</h4>
        <li class="btn-wide">
            <label class="label cursor-pointer">
            <span class="label-text">{$i18n.t("Dark mode")}</span> 
            <input type="checkbox" class="toggle" checked={$theme === Theme.Dark} on:change={()=>{
                $theme = $theme === Theme.Dark ? Theme.Light : Theme.Dark;
            }} />
        </label></li>
        <li class="btn-wide">
            <label class="label cursor-pointer">
            <span class="label-text">{ $i18n.t("Scale") }</span>
            <select class="select select-sm w-full max-w-xs" bind:value={$scale}>
            {#each scales as scaleOption}
                <option value={scaleOption}>{Math.ceil(scaleOption * 100) + "%"}</option>
            {/each}
        </label></li>
        <li class="btn-wide">
            <label class="label cursor-pointer">
            <span class="label-text">{ $i18n.t("Locale") }</span>
            <select class="select select-sm w-full max-w-xs" bind:value={$locale}>
                {#each locales as localeOption}
                    <option value={localeOption}>{ $i18n.t(localeOption) }</option>
                {/each}
        </label></li>
        <li class="btn-wide"><a href={null} on:click={() => { openModal("about_modal"); }}>
            {@html aboutIcon} { $i18n.t("About") }
        </a></li>
    </ul>
</div>
