<script lang="ts">
import { onMount, onDestroy, afterUpdate } from 'svelte';

import MavlinkEdit from './MavlinkEdit.svelte';
import ConnectionChart from './ConnectionChart.svelte';

import { type LinkDescription, type LinkStatus } from "$bindings/communication";
import { getLinkStatus, saveLink, removeLink, setLinkConnected } from "$stores/communication";

import { i18n } from "$stores/i18n";

export let selectedLinkId = ""

export let link: LinkDescription
export let changed: boolean = false

let linkCopy: LinkDescription = link
let status: LinkStatus | null = null

let interval: any

onMount(async () => {
    interval = setInterval(async () => { status = link.id ? await getLinkStatus(link.id) : null; }, 250);
})

onDestroy(async () => { clearInterval(interval); });

afterUpdate(async () => {
    if (selectedLinkId !== link.id) {
        linkCopy = link;
    } else {
        changed = JSON.stringify(linkCopy) !== JSON.stringify(link);
    }
});

</script>

<div class="collapse collapse-arrow bg-base-200">
    <input type="radio" name="communication-links-accordion" on:change={() => { selectedLinkId = link.id || "" }}/> 
    <div class="collapse-title flex flex-row gap-4">
        <div class="indicator w-full">
            <span class={"indicator-item badge badge-xs indicator-start indicator-middle " +
                (status && status?.is_connected ? status?.is_online ? "bg-success" : "bg-warning" : "bg-neutral-content")} >
            </span>
            <h1 class="font-medium ml-8 my-2">{link.name}</h1>
            <ConnectionChart status={status} />
        </div>
        <div class="join btn-sm p-0 z-[1]">
            <button class="btn btn-sm btn-ghost px-1 join-item" disabled={ status?.is_connected || changed }
                on:click={() => { setLinkConnected(link.id || "", true) }}>
                { $i18n.t("Connect") }
            </button>
            <button class="btn btn-sm btn-ghost px-1 join-item" disabled={ !status?.is_connected }
                on:click={() => { setLinkConnected(link.id || "", false) }}>
                { $i18n.t("Disconnect") }
            </button>
        </div>
    </div>
    <div class="collapse-content gap-8">
        <div class="grid grid-cols-2 gap-2">
            <!-- Name -->
            <h1 class="font-medium my-2 w-full">{ $i18n.t("Name") }</h1>
            <input type="text" placeholder={ $i18n.t("Name cannot be empty") } class="input w-full"
                bind:value={linkCopy.name}/>

            <!-- Protocol -->
            {#if linkCopy.protocol.Mavlink}
                <MavlinkEdit bind:protocol={linkCopy.protocol.Mavlink} disabled={ status?.is_connected == true }/>
            {/if}
        </div>

        <div class="w-full btn-sm mt-4 flex">
            <button disabled={status?.is_connected} class="btn btn-sm btn-wide btn-secondary px-1 ml-2"
                on:click={() => { removeLink(link.id || "") }}>
                { $i18n.t("Remove") }
            </button>

            <div class="grow"/>

            <div class="join btn-sm p-0">
                <button disabled={!changed} class="btn btn-sm btn-wide btn-primary join-item px-1 ml-2"
                    on:click={()=> { linkCopy = link }}>
                    { $i18n.t("Discard") }
                </button>
                <button disabled={!changed} class="btn btn-sm btn-wide btn-accent join-item px-1 ml-2"
                    on:click={ async () => { link = await saveLink(linkCopy) || link }}>
                    { $i18n.t("Save") }
                </button>
            </div>
        </div>

            <!-- TODO: UNDO, SAVE -->
    </div>
</div>