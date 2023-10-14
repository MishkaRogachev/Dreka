<script lang="ts">

import CommunicationLink from "$pages/communication/CommunicationLink.svelte";

import { closeModal } from "$stores/app";
import { links } from "$stores/communication";
import { i18n } from "$stores/i18n";

import closeIcon from "$assets/svg/close.svg?raw";

let expandedIndex = -1

</script>

<style>
#communication {
    position: fixed;
    top: 48px;
    left: 16px;
    width: 45%;
    min-height: 20%;
    max-height: 80%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 8px;
}

#header {
    width: 100%;
    display: flex;
    flex-direction: row;
    text-align: center;
    align-items: center;
}
</style>

<div id="communication" class="card rounded-box bg-base-100 shadow-xl p-2">
    <div id="header" class="container">
        <button class="btn btn-ghost btn-sm rounded-btn px-0" on:click={closeModal}>{@html closeIcon}</button>
        <h1 class="text-xl font-bold px-2">{ $i18n.t("Communication") }</h1>
    </div>
    {#each $links as link, i}
    <div class="collapse bg-base-200">
        <input type="radio" name="my-accordion-2" />
        <div class="collapse-title flex flex-row gap-4">
            <span class="indicator-item badge" ></span> 
            <!-- TODO: status -->
            <h1 class="text-l font-medium">{link.name}</h1>
        </div>
        <div class="collapse-content"> 
            <CommunicationLink
            link={link}
            expanded={expandedIndex === i}
            on:expand = {() => { expandedIndex = i; }}
            on:collapse = {() => { expandedIndex = -1; }}/>
        </div>
    </div>
    {/each}
    <!-- BUTTON ADD NEW -->
    <!-- CHECKBOX send heartbeat -->
</div>