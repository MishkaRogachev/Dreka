<script lang="ts">
import Label from "$components/controls/Label.svelte";
import CommunicationLink from "$pages/settings/Communication/CommunicationLink.svelte";

import { communicationLinks } from "$stores/links";
</script>

<style>
#communication {
    width: 100%;
    min-height: 50%;
    max-height: 100%;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px;
}
</style>

<div id="communication" class="panel">
    <Label text="Communication Links" />
    {#each $communicationLinks as link}
        <CommunicationLink link={link}/>
    {/each}
</div>