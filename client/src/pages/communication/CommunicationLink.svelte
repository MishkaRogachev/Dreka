<script lang="ts">
import { onMount, onDestroy, createEventDispatcher } from 'svelte';

import TextEdit from '$components/controls/TextEdit.svelte';
import Button from "$components/controls/Button.svelte";
import Label from "$components/controls/Label.svelte";
import Led from "$components/controls/Led.svelte";

import { type LinkDescription, type LinkProtocol, MavlinkProtocolVersion, type LinkStatus } from "$bindings/communication";
import { getLinkStatus, saveLink } from "$stores/communication";

import upIcon from "$assets/svg/up.svg";
import downIcon from "$assets/svg/down.svg";

export let link: LinkDescription
export let expanded: boolean = false

let status: LinkStatus | null = null

let interval: any
let blocked: boolean = false

const dispatch = createEventDispatcher()

function expand() { dispatch('expand', {}); }
function collapse() { dispatch('collapse', {}); }

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

<style>
#link {
    width: 100%;
    min-height: 42px;
    gap: 8px;
    display: flex;
    flex-direction: column;
    justify-content: center;
}

.row {
    width: 100%;
    margin-top: 4px;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
}

.header-buttons {
    width: 20%;
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
}
</style>

<div id="link" class="frame">

<!-- Link Preview -->
<div class="row">
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
</div>

<!-- Link Editor -->
{#if expanded}
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

</div>