<script lang="ts">

import { MavlinkProtocolVersion, type LinkType, type MavlinkProtocol } from "$bindings/communication";

import { i18n } from "$stores/i18n";

export let protocol: MavlinkProtocol

const availableProtocols = [ MavlinkProtocolVersion.MavlinkV1, MavlinkProtocolVersion.MavlinkV2 ];

// $: connections = getProtocolConnections(link.protocol)
// $: current_protocol = getCurrentProtocolVersion(link.protocol)
// $: available_protocols = getAvailableProtocolVersions(link.protocol)


// export function getProtocolConnections(protocol: LinkProtocol): Array<SocketData | SerialData> {
//     let connections: Array<SocketData | SerialData> = [];

//     if (protocol.Mavlink !== null ) {
//         console.log("--->", protocol.Mavlink.link_type);
//     }

//     return connections;
// }

</script>

<!-- PROTOCOL VERSION -->
<h1 class="font-medium my-2 w-full">{ $i18n.t("Protocol") }</h1>
<select class="select" value={protocol.protocol_version}>
{#each availableProtocols as protocol}
    <option value={protocol}>{ $i18n.t(protocol) }</option>
{/each}
</select>

<!-- TCP SOCKET ADDRESS & PORT -->
{#if protocol.link_type.Tcp}
    <h1 class="font-medium my-2 w-full">{ $i18n.t("TCP Address") }</h1>
    <input type="text" placeholder={ $i18n.t("Address cannot be empty") } class="input w-full"
        bind:value={protocol.link_type.Tcp.address}/>
    <h1 class="font-medium my-2 w-full">{ $i18n.t("TCP Port") }</h1>
    <input type="number" placeholder={ $i18n.t("Port cannot be empty") } class="input w-full"
        bind:value={protocol.link_type.Tcp.port}/>
{/if}

<!-- TCP SOCKET ADDRESS & PORT -->
{#if protocol.link_type.Udp}
    <h1 class="font-medium my-2 w-full">{ $i18n.t("UDP Address") }</h1>
    <input type="text" placeholder={ $i18n.t("Address cannot be empty") } class="input w-full"
        bind:value={protocol.link_type.Udp.address}/>
    <h1 class="font-medium my-2 w-full">{ $i18n.t("UDP Port") }</h1>
    <input type="number" placeholder={ $i18n.t("Port cannot be empty") } class="input w-full"
        bind:value={protocol.link_type.Udp.port}/>
{/if}

<!-- SERIAL PORT ADDRESS & BAUD RATE -->
{#if protocol.link_type.Serial}
    <h1 class="font-medium my-2 w-full">{ $i18n.t("Serial Port") }</h1>
    <input type="text" placeholder={ $i18n.t("Port cannot be empty") } class="input w-full"
        bind:value={protocol.link_type.Serial.port}/>
    <!-- <h1 class="font-medium my-2 w-full">{ $i18n.t("baud rate") }</h1> -->
    <!-- TODO: baud rete -->
{/if}
