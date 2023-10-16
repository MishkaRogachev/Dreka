<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import MavlinkEdit from './MavlinkEdit.svelte';

import { type LinkDescription, type LinkStatus } from "$bindings/communication";
import { getLinkStatus, saveLink } from "$stores/communication";

import { i18n } from "$stores/i18n";

export let link: LinkDescription

let status: LinkStatus | null = null

let interval: any
let blocked: boolean = false

onMount(async () => {
    interval = setInterval(async () => { status = link.id ? await getLinkStatus(link.id) : null; }, 250);
})

onDestroy(async () => { clearInterval(interval); });


async function setLinkEnabled(link: LinkDescription, enabled: boolean) {
    link.enabled = enabled;
    blocked = true;

    let linkBack = await saveLink(link);
    if (linkBack) {
        link = linkBack;
        blocked = false;
    }
}

</script>

<div class="collapse collapse-arrow bg-base-200">
    <input type="radio" name="my-accordion-2" />
    <div class="collapse-title flex flex-row gap-4">
        <div class="indicator w-full">
            <span class={"indicator-item badge badge-xs indicator-start indicator-middle " + 
                (status && status?.is_connected ? status?.is_online ? "bg-success" : "bg-warning" : "bg-neutral-content")} >
            </span>
            <h1 class="font-medium ml-8 my-2">{link.name}</h1>
        </div>
        <div class="join btn-sm p-0 z-[1]">
            <button class="btn btn-sm btn-ghost px-1 join-item" disabled={ link.enabled || blocked }
                on:click={() => { setLinkEnabled(link, true) }}>
                { $i18n.t("Connect") }
            </button>
            <button class="btn btn-sm btn-ghost px-1 join-item" disabled={ !link.enabled || blocked }
                on:click={() => { setLinkEnabled(link, false) }}>
                { $i18n.t("Disconnect") }
            </button>
        </div>
    </div>
    <div class="collapse-content grid grid-cols-2 gap-2">
        <!-- Name -->
        <h1 class="font-medium my-2 w-full">{ $i18n.t("Name") }</h1>
        <input type="text" placeholder={ $i18n.t("Name cannot be empty") } class="input w-full"
            bind:value={link.name}/>

        <!-- Protocol -->
        {#if link.protocol.Mavlink}
            <MavlinkEdit protocol={link.protocol.Mavlink} />
        {/if}

        <!-- TODO: REMOVE, UNDO, SAVE -->
    </div>
</div>