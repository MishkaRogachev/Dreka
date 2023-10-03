<script lang="ts">
import Button from "$components/controls/Button.svelte";
import Label from "$components/controls/Label.svelte";
import Led from "$components/controls/Led.svelte";

import { type LinkDescription, type LinkProtocol, MavlinkProtocolVersion } from "$bindings/links";

import connectIcon from "$assets/svg/connect.svg"
import disconnectIcon from "$assets/svg/disconnect.svg"

export let link: LinkDescription

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
    return name
}
</script>

<style>
#link {
    width: 100%;
    min-height: 48px;
    display: flex;
    flex-direction: row;
    gap: 8px;
    justify-content: space-between;
}
</style>

<div id="link" class="frame">
    <Led />
    <!-- Link status -->
    <Label text={link.name} style="width: 256px;"/>
    <Label text={getProtocolName(link.protocol)} style="width: 256px;"/>
    <div>
        <Button disabled={link.enabled} text="Connect" right_cropped={true}/>
        <Button disabled={!link.enabled} text="Disconnect" left_cropped={true}/>
    </div>
</div>