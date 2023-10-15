<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import { type LinkDescription, type LinkProtocol, MavlinkProtocolVersion, type LinkStatus } from "$bindings/communication";
import { getLinkStatus, saveLink } from "$stores/communication";

import { i18n } from "$stores/i18n";

export let link: LinkDescription

let status: LinkStatus | null = null

let interval: any
let blocked: boolean = false

onMount(async () => {
    interval = setInterval(async () => { status = await getLinkStatus(link.id); }, 250);
})

onDestroy(async () => { clearInterval(interval); });

function getProtocolName(protocol: LinkProtocol): string {
    let name: string = "";
    if (protocol.Mavlink !== null ) {
        name += "MAVLink";
        switch (protocol.Mavlink.protocol_version) {
            case MavlinkProtocolVersion.MavlinkV1:
                name += ":V1";
                break;
            case MavlinkProtocolVersion.MavlinkV2:
                name += ":V2";
                break;
            default:
                break;
        }
    }
    return name;
}

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

<!-- Link Preview -->
<!-- <div class="row">
    <Led state={ status && status?.is_connected ? status?.is_online ? "on" : "warning" : "off"}/>
    <Label text={link.name} style="width: 256px;"/>
    <Label text={getProtocolName(link.protocol)} style="width: 256px;"/>
    <div class="header-buttons">
        {#if !expanded}
        <Button disabled={link.enabled || blocked} text="Connect" right_cropped={true}
            on:click={() => { setLinkEnabled(link, true) }}/>
        <Button disabled={!link.enabled || blocked} text="Disconnect" right_cropped={true} left_cropped={true}
            on:click={() => { setLinkEnabled(link, false) }}/>
        {/if}
        <Button left_cropped={expanded} icon={ expanded ? upIcon : downIcon }
            on:click={expanded ? collapse : expand }/>
    </div>
</div> -->

<!-- Link Editor -->
<!-- {#if expanded}
<table style="width: 256px">
    <colgroup>
        <col span="1" style="width: 35%;">
        <col span="1" style="width: 65%;">
    </colgroup>
    <tr><td>
        <Label text="Name"/>
    </td>
    <td>
        <TextEdit style="width:100%"/>
    </td></tr>
</table>
{/if}

</div> -->


<div class="collapse bg-base-200">
    <input type="radio" name="my-accordion-2" />
    <div class="collapse-title flex flex-row gap-4">
        <div class="indicator">
            <span class={"indicator-item badge badge-xs indicator-start indicator-middle " + 
                (status && status?.is_connected ? status?.is_online ? "bg-success" : "bg-warning" : "bg-neutral-content")} >
            </span>
            <h1 class="text-l font-medium ml-8">{link.name}</h1>
        </div>
    </div>
    <div class="collapse-content container">
        link.id
        <!-- TODO: REMOVE -->
        <div class="join btn-sm p-0 w-full">
            <button class="btn btn-sm px-1 join-item" disabled={ link.enabled || blocked }
                on:click={() => { setLinkEnabled(link, true) }}>
                { $i18n.t("Connect") }
            </button>
            <button class="btn btn-sm px-1 join-item" disabled={ !link.enabled || blocked }
                on:click={() => { setLinkEnabled(link, false) }}>
                { $i18n.t("Disconnect") }
            </button>
        </div>
    </div>
</div>