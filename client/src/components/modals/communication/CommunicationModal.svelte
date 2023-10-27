<script lang="ts">
import BaseModal from "$components/common/BaseModal.svelte";
import CommunicationLink from "./CommunicationLink.svelte";

import { type LinkDescription, MavlinkProtocolVersion } from "$bindings/communication";

import { all_links, saveLink } from "$stores/communication";
import { i18n } from "$stores/i18n";

export let selectedLinkId = ""

const linksForCreation: Array<LinkDescription> = [
    {
        autoconnect: false,
        name: $i18n.t("New UDP Link"),
        protocol: {
            Mavlink: {
                link_type: {
                    Udp: {
                        address: "127.0.0.1",
                        port: 14550
                    },
                },
                protocol_version: MavlinkProtocolVersion.MavlinkV2
            }
        }
    },
    {
        autoconnect: false,
        name: $i18n.t("New TCP Link"),
        protocol: {
            Mavlink: {
                link_type: {
                    Tcp: {
                        address: "127.0.0.1",
                        port: 5670
                    },
                },
                protocol_version: MavlinkProtocolVersion.MavlinkV2
            }
        }
    },
    {
        autoconnect: false,
        name: $i18n.t("New Serial Link"),
        protocol: {
            Mavlink: {
                link_type: {
                    Serial: {
                        port: "",
                        baud_rate: 115200
                    },
                },
                protocol_version: MavlinkProtocolVersion.MavlinkV2
            }
        }
    }
]

function closeDropdown() {
    document.getElementById("newCommunicationLinkDropdown")?.removeAttribute("open");
}

</script>

<BaseModal id="communication_modal">
    <form method="dialog">
        <!-- CLOSE -->
        <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
        <!-- ADD NEW -->
        <details id="newCommunicationLinkDropdown" class="dropdown absolute left-2 top-2">
            <summary class="btn m-1">{ $i18n.t("Add Link") }</summary>
            <ul class="dropdown-content z-[3] menu p-2 shadow bg-base-300 rounded-box w-48">
                {#each linksForCreation as link}
                    <li on:click={() => { saveLink(link); closeDropdown(); }}><a>{ link.name }</a></li>
                {/each}
            </ul>
        </details>
    </form>
    <h3 class="font-bold text-lg text-center mb-4">{ $i18n.t("Communication Links") }</h3>

    <!-- LIST COMPONENT -->
    <div class="my-4 space-y-2 max-scroll-area-height overflow-y-auto">
    {#each $all_links.values() as link}
        <CommunicationLink link={link} bind:selectedLinkId={selectedLinkId}/>
    {/each}
    </div>

    <!-- FILLER -->
    <div class="flex flex-col grow text-center">
    {#if $all_links.size === 0}
        <a class="grow">{ $i18n.t("No communication links available") }</a>
    {:else}
        <div class="grow"/>
    {/if}
    </div>

    <!-- SEND GCS HEARTBEAT TODO: backend -->
    <div class="form-control grow-0">
        <label class="label cursor-pointer">
            <span class="label-text">{ $i18n.t("Send GCS heartbeat") }</span>
            <input type="checkbox" class="checkbox" />
        </label>
    </div>
</BaseModal>