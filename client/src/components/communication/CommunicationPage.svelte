<script lang="ts">
import CommunicationLink from "$components/communication/CommunicationLink.svelte";

import { type LinkDescription, MavlinkProtocolVersion } from "$bindings/communication";

import { links, saveLink } from "$stores/communication";
import { i18n } from "$stores/i18n";

const linksForCreation: Array<LinkDescription> = [
    {
        enabled: false,
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
        enabled: false,
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
        enabled: false,
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

</script>

<dialog id="communication_modal" class="modal">
    <div class="modal-box w-11/12 max-w-5xl">
        <form method="dialog">
            <!-- CLOSE -->
            <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
            <!-- ADD NEW -->
            <div class="dropdown absolute left-2 top-2">
                <label tabindex="0" class="btn m-1">{ $i18n.t("New Link") }</label>
                <ul tabindex="0" class="dropdown-content z-[3] menu p-2 shadow bg-base-300 rounded-box w-48">
                    {#each linksForCreation as link, i}
                        <li on:click={() => { saveLink(link); }} value={i}><a>{ link.name }</a></li>
                    {/each}
                </ul>
            </div>
        </form>
        <h3 class="font-bold text-lg text-center mb-4">{ $i18n.t("Communication Links") }</h3>

        <div class="grid gap-y-2 my-4">
        {#each $links as link, i}
            <CommunicationLink link={link}/>
        {/each}
        </div>

        <!-- SEND GCS HEARTBEAT TODO: backend -->
        <div class="form-control">
            <label class="label cursor-pointer">
                <span class="label-text">{ $i18n.t("Send GCS heartbeat") }</span> 
                <input type="checkbox" class="checkbox" />
            </label>
        </div>
    </div>
</dialog>