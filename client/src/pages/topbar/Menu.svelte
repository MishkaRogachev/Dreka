<script lang="ts">
import { Page, currentPage, Theme, theme, scale, scales } from '$stores/app';
import { locale, locales } from '$stores/i18n';

import { i18n } from '$stores/i18n';

import burgerIcon from "$assets/svg/burger.svg?raw";
import fleetIcon from "$assets/svg/fleet.svg?raw";
import connectIcon from "$assets/svg/connect.svg?raw";
import aboutIcon from "$assets/svg/about.svg?raw";

</script>

<div class="dropdown dropdown-hover">
    <label tabindex="0" class="btn btn-ghost btn-sm rounded-btn">{@html burgerIcon}</label>
    <ul tabindex="0" class="dropdown-content menu z-[1] p-2 shadow bg-base-100 rounded-box my-0">
        <h4>{ $i18n.t("Main") }</h4>
        <li class="btn-wide"><a on:click={() => {$currentPage = Page.Flight}}>{@html fleetIcon} { $i18n.t("Flight") }</a></li>
        <h4>{ $i18n.t("Configure") }</h4>
        <li class="btn-wide"><a on:click={() => {$currentPage = Page.Communication}}>{@html connectIcon} { $i18n.t("Communication") }</a></li>
        <h4>{ $i18n.t("Misc") }</h4>
        <li class="btn-wide"><label class="label cursor-pointer">
            <span class="label-text">{$i18n.t("Dark mode")}</span> 
            <input type="checkbox" class="toggle" checked={$theme === Theme.Dark} on:change={()=>{
                $theme = $theme === Theme.Dark ? Theme.Light : Theme.Dark }} />
        </label></li>
        <li class="btn-wide"><label class="label cursor-pointer">
            <span class="label-text">{ $i18n.t("Scale") }</span>
            <select class="select select-bordered select-sm w-full max-w-xs" bind:value={$scale}>
                {#each scales as scaleOption}
                    <option value={scaleOption}>{Math.ceil(scaleOption * 100) + "%"}</option>
                {/each}
        </label></li>
        <li class="btn-wide"><label class="label cursor-pointer">
            <span class="label-text">{ $i18n.t("Locale") }</span>
            <select class="select select-bordered select-sm w-full max-w-xs" bind:value={$locale}>
                {#each locales as localeOption}
                    <option value={localeOption}>{ $i18n.t(localeOption) }</option>
                {/each}
        </label></li>
        <!-- TODO: language -->
        <li class="btn-wide"><a on:click={() => {$currentPage = Page.About}}>{@html aboutIcon} { $i18n.t("Scale") }</a></li>
    </ul>
</div>
