
<script lang="ts">
import { onMount, onDestroy } from 'svelte';

import { activeDialog } from "$stores/app";

export function closeDialog() {
    $activeDialog = undefined;
}

function keyListener(event: KeyboardEvent) {
    if (event.key === "Escape") {
        closeDialog();
    }
}

onMount(async () => {
    document.addEventListener("keydown", keyListener);
});

onDestroy(() => {
    document.removeEventListener("keydown", keyListener);
});

</script>

<style>
#commandDialog {
    left: 50%;
    transform: translateX(-50%);
}
</style>

<div id="commandDialog" class="absolute top-10 bg-base-300 p-2 rounded-md shadow-lg">
    <div class="flex grow">
        <h3 class="font-bold text-lg text-center grow mb-4"><slot name="title"/></h3>
        <button class="btn btn-sm btn-circle btn-ghost" on:click={closeDialog}>✕</button>
    </div>
    <slot name="content"/>
</div>
