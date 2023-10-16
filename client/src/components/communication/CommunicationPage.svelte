<script lang="ts">

import CommunicationLink from "$components/communication/CommunicationLink.svelte";

import { links } from "$stores/communication";
import { i18n } from "$stores/i18n";

import closeIcon from "$assets/svg/close.svg?raw";

</script>

<dialog id="communication_modal" class="modal">
    <div class="modal-box w-11/12 max-w-5xl">
        <form method="dialog">
            <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
        </form>
        <h3 class="font-bold text-lg text-center mb-4">{ $i18n.t("Communication") }</h3>
        <div class="grid gap-y-2">
        {#each $links as link, i}
            <CommunicationLink link={link}/>
        {/each}
        </div>
        <!-- BUTTON ADD NEW -->
        <!-- CHECKBOX send heartbeat -->
    </div>
</dialog>