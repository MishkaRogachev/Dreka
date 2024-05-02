
<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import { activeDialog } from "$stores/app";

export function closeDialog() { $activeDialog = undefined; }

export let windowX = window.innerWidth / 3;
export let windowY = 40;

let minX = 8; // offset
let minY = 40; // offset + topbar
let dX = 0;
let dY = 0;

let isMoving = false;
let onDown: any = undefined;
let onUp: any = undefined;
let onMove: any = undefined;

function keyListener(event: KeyboardEvent) {
    if (event.key === "Escape") {
        closeDialog();
    }
}

function handle(node: any) {
    node.style.userSelect = 'none';

    onDown = (event: MouseEvent) => { 
        isMoving = true;
        dX = windowX - event.clientX;
        dY = windowY - event.clientY;
    };
    node.addEventListener("mousedown", onDown);

    onUp = () => { isMoving = false; };
    window.addEventListener("mouseup", onUp);

    onMove = (event: MouseEvent) => {
        if (isMoving) {
            let maxX = window.innerWidth - 32;
            let maxY = window.innerHeight - 8;

            windowY = Math.min(maxY - node.clientHeight, Math.max(minY, event.clientY + dY));

            const dashboard = document.getElementById('dashboard');
            if (dashboard && windowY < dashboard.clientHeight + 32) {
                maxX -= dashboard.clientWidth;
            }

            windowX = Math.min(maxX - node.clientWidth, Math.max(minX, event.clientX + dX));
        }
    };
    window.addEventListener("mousemove", onMove);
}

onMount(async () => {
    document.addEventListener("keydown", keyListener);
});

onDestroy(() => {
    document.removeEventListener("keydown", keyListener);
    if (onUp) {
        window.removeEventListener("mouseup", onUp);
    }
    if (onMove) {
        window.removeEventListener("mousemove", onMove);
    }
});

</script>

<!-- TODO: movable -->
<div class="absolute top-10 bg-base-300 p-2 rounded-md shadow-lg"
    style="left:{windowX}px; top:{windowY}px">
    <div class={"flex grow " + (isMoving ? "cursor-grabbing" : "cursor-grab") } use:handle >
        <h3 class="font-bold text-lg text-center grow mb-4"><slot name="title"/></h3>
        <button class="btn btn-sm btn-circle btn-ghost" on:click={closeDialog}>✕</button>
    </div>
    <slot name="content"/>
</div>
