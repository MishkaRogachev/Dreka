<script lang="ts">
import { onMount, onDestroy } from 'svelte';
import { MavlinkProtocolVersion, type MavlinkProtocol } from "$bindings/communication";
import { CommunicationService } from "$services/communication";

import { i18n } from "$stores/i18n";

export let disabled: boolean
export let protocol: MavlinkProtocol

const protocolVersions = [ MavlinkProtocolVersion.MavlinkV1, MavlinkProtocolVersion.MavlinkV2 ];

var serialInterval: NodeJS.Timeout;
var availablePorts: Array<string> = [];
var baudRates: Array<number> = [];

async function updateSerialPorts() {
    availablePorts = await CommunicationService.getAvaliableSerialPorts() || [];
    baudRates = await CommunicationService.getAvaliableBaudRates() || [];
}

onMount(async () => {
    serialInterval = setInterval(() => { updateSerialPorts() }, 10000);
    updateSerialPorts();
});

onDestroy(() => {
    clearInterval(serialInterval);
});


</script>

<!-- PROTOCOL VERSION -->
<h1 class="font-medium my-2 w-full">{ $i18n.t("Protocol") }</h1>
<select disabled={disabled} class="select" bind:value={protocol.protocol_version}>
{#each protocolVersions as protocol_version}
    <option value={protocol_version}>{ $i18n.t(protocol_version) }</option>
{/each}
</select>

<!-- TCP SOCKET ADDRESS & PORT -->
{#if protocol.link_type.Tcp}
    <h1 class="font-medium my-2 w-full">{ $i18n.t("TCP Address") }</h1>
    <input disabled={disabled} type="text" placeholder={ $i18n.t("Address cannot be empty") } class="input w-full"
        bind:value={protocol.link_type.Tcp.address}/>
    <h1 class="font-medium my-2 w-full">{ $i18n.t("TCP Port") }</h1>
    <input disabled={disabled} type="number" placeholder={ $i18n.t("Port cannot be empty") } class="input w-full"
        bind:value={protocol.link_type.Tcp.port}/>
{/if}

<!-- TCP SOCKET ADDRESS & PORT -->
{#if protocol.link_type.Udp}
    <h1 class="font-medium my-2 w-full">{ $i18n.t("UDP Address") }</h1>
    <input disabled={disabled} type="text" placeholder={ $i18n.t("Address cannot be empty") } class="input w-full"
        bind:value={protocol.link_type.Udp.address}/>
    <h1 class="font-medium my-2 w-full">{ $i18n.t("UDP Port") }</h1>
    <input disabled={disabled} type="number" placeholder={ $i18n.t("Port cannot be empty") } class="input w-full"
        bind:value={protocol.link_type.Udp.port}/>
{/if}

<!-- SERIAL PORT ADDRESS & BAUD RATE -->
{#if protocol.link_type.Serial}
    <h1 class="font-medium my-2 w-full">{ $i18n.t("Serial Port") }</h1>
    <select disabled={disabled} class="select" bind:value={protocol.link_type.Serial.port}>
        {#each availablePorts as port}
            <option value={port}>{ $i18n.t(port) }</option>
        {/each}
    </select>
    <h1 class="font-medium my-2 w-full">{ $i18n.t("Baud Rate") }</h1>
    <select disabled={disabled} class="select" bind:value={protocol.link_type.Serial.baud_rate}>
        {#each baudRates as baudRate}
            <option value={baudRate}>{ baudRate }</option>
        {/each}
    </select>
{/if}
