<script lang="ts">
import { afterUpdate } from 'svelte';

import MavlinkEdit from './MavlinkEdit.svelte';
import ConnectionChart from './ConnectionChart.svelte';

import { type LinkDescription } from "$bindings/communication";
import { Link, links } from "$stores/communication";

import { i18n } from "$stores/i18n";

export let selectedLinkId = "";
export let link: Link;

let descriptionCopy: LinkDescription = cloneDescription();
let changed = false;

afterUpdate(async () => {
    if (selectedLinkId !== link.description.id) {
        descriptionCopy = cloneDescription();
    } else {
        changed = JSON.stringify(descriptionCopy) !== JSON.stringify(link.description);
    }
});

function cloneDescription() {
    return JSON.parse(JSON.stringify(link.description));
}
</script>

<div class={"collapse collapse-arrow bg-base-200"}>
    <input type="radio" checked={ selectedLinkId === link.description.id } name="communication-links-accordion"
        on:change={() => { selectedLinkId = link.description.id }}/>
    <div class="collapse-title flex flex-row gap-4">
        <div class="indicator w-full">
            <span class={"indicator-item badge badge-xs indicator-start indicator-middle " +
                (link.status?.is_enabled ? link.status?.is_connected ? link.status?.is_online ? "bg-success" : "bg-warning": "bg-error" : "bg-neutral-content")} >
            </span>
            <h1 class="font-medium ml-8 my-2 btn-wide text-left">{link.description.name}</h1>
            <ConnectionChart status={link.status} />
        </div>
        <div class="join btn-sm p-0 z-[1]">
            <button class="btn btn-sm btn-ghost px-1 join-item" disabled={ link.status?.is_enabled || changed }
                on:click={() => { links.setLinkEnabled(link.description.id, true) }}>
                { $i18n.t("Enable") }
            </button>
            <button class="btn btn-sm btn-ghost px-1 join-item" disabled={ !link.status?.is_enabled }
                on:click={() => { links.setLinkEnabled(link.description.id, false) }}>
                { $i18n.t("Disable") }
            </button>
        </div>
    </div>
    <div class="collapse-content gap-8">
        <div class="grid grid-cols-2 gap-2">
            <!-- Name -->
            <h1 class="font-medium my-2 w-full">{ $i18n.t("Name") }</h1>
            <input type="text" placeholder={ $i18n.t("Enter name here") } class="input w-full"
                bind:value={descriptionCopy.name}/>

            <!-- Protocol -->
            {#if descriptionCopy.protocol.Mavlink}
                <MavlinkEdit bind:protocol={descriptionCopy.protocol.Mavlink} disabled={ link.status?.is_enabled == true }/>
            {/if}
        </div>

        <div class="w-full btn-sm mt-4 flex">
            <button disabled={link.status?.is_enabled} class="btn btn-sm btn-wide btn-secondary btn-outline px-1 ml-2"
                on:click={() => { links.removeLink(link.description.id) }}>
                { $i18n.t("Remove") }
            </button>

            <div class="grow"/>

            <div class="join btn-sm p-0">
                <button disabled={!changed} class="btn btn-sm btn-wide btn-primary join-item px-1 ml-2"
                    on:click={()=> { descriptionCopy = cloneDescription() }}>
                    { $i18n.t("Discard") }
                </button>
                <button disabled={!changed} class="btn btn-sm btn-wide btn-accent join-item px-1 ml-2"
                    on:click={ async () => { link = await links.saveLink(descriptionCopy) || link }}>
                    { $i18n.t("Save") }
                </button>
            </div>
        </div>
    </div>
</div>
