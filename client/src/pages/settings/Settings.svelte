<script lang="ts">
import Button from "$components/controls/Button.svelte";
import Communication from "$pages/settings/Communication/Communication.svelte";

export let visible: boolean = true

let settingsPages = ["Communication", "About"]
let selectedPage: number = 0

</script>

<style>
#settingsPanel {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: row;
    gap: 16px;
}

#buttons {
    width: 20%;
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 16px;
    margin-left: 8px;
}

#content {
    margin-top: 2%;
    width: 75%;
    height: 90%;
    display: flex;
    align-items: center;
}
</style>

<div id="settingsPanel" style={visible ? "" : "display: none"}>
    <div id="buttons">
    {#each settingsPages as page, index}
        <Button style="width:100%" selected={index == selectedPage} text={page} on:click={() => { selectedPage = index }}/>
    {/each}
    </div>
    <div id="content">
        {#if selectedPage === 0}
            <Communication />
        {/if}
    </div>
</div>

