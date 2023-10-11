<script lang="ts">
import Label from "$components/controls/Label.svelte";
import CommunicationLink from "$pages/communication/CommunicationLink.svelte";

import { links } from "$stores/communication";

let expandedIndex = -1

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
    {#each $links as link, i}
        <CommunicationLink
            link={link}
            expanded={expandedIndex === i}
            on:expand = {() => { expandedIndex = i; }}
            on:collapse = {() => { expandedIndex = -1; }}/>
    {/each}
</div>