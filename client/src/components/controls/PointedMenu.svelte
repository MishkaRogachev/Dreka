<script lang="ts">
import { onMount } from "svelte"

import type { ICartesian } from "$lib/common/common-interfaces"

export let rootXY: ICartesian = { x: 0, y: 0 }
export let isOpen: boolean = false
export let pointerSize: number = 10

export function open(pos: ICartesian) {
    rootXY = pos
    isOpen = true
}

export function close() {
    isOpen = false
}

let menuWidth: number
let menuHeight: number

onMount(async () => {
    document.addEventListener("keydown", (event: any) => {
        if (event.key === "Escape") {
            isOpen = false
        }
    })
})
</script>

<style>
#menu {
    position: absolute;
    min-width: 128px
}
#pointer {
    position: absolute;
    width: 0;
    height: 0;
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;
    border-top: 10px solid rgba(0, 0, 0, 0.85);
}
</style>

{#if isOpen}
<div id="menu" class="pane" style="top: {rootXY.y - menuHeight - pointerSize}px; left: {rootXY.x - menuWidth / 2}px;"
    bind:clientWidth={menuWidth} bind:clientHeight={menuHeight}>
    <slot></slot>
</div>
<div id="pointer" style="top: {rootXY.y - pointerSize}px; left: {rootXY.x - pointerSize / 2}px;"></div>
{/if}
