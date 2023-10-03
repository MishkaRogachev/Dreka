<script lang="ts">
 import { createEventDispatcher } from "svelte";

import Button from "$components/controls/Button.svelte";
import OverlayButton from "./OverlayButton.svelte";

export let items: Array<string>;
export let selectedIndex: number = -1;
export let placeholder: string = "No items";

$: selectedItem = selectedIndex >= 0 && selectedIndex < items.length ? items[selectedIndex] : null;

let menuComponent: any

const dispatch = createEventDispatcher()

function selected(index: number) { dispatch('selected', { index: index }); }
</script>

<OverlayButton
    bind:this={menuComponent}
    style="width:112px;"
    disabled={items.length === 0}
    text={selectedItem ? selectedItem : placeholder}
    flat={true}
    opacity={false}
    margin = {24}
    position="bottom-center">
    <div style="width:96px; max-height:256px">
    {#each items as item, index}
        <Button style="width:100%" flat={true} disabled={index == selectedIndex} text={item}
            on:click={() => { selectedIndex = index; selected(selectedIndex); menuComponent.close() }}/>
    {/each}
    </div>
</OverlayButton>
