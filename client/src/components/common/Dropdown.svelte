<script lang="ts">
import { onMount, onDestroy } from 'svelte';
import { clickOutside } from '$lib/common/click-outside';

import { closeAllPopups } from '$stores/app';

export let tip: string = "";
export let empty: boolean = false;

let contentHeight: number = 0;

const uuid = window.crypto.randomUUID()

export function closeDropdown() {
    document.getElementById(uuid)?.removeAttribute("open");
}

let isOpen: boolean = false;

function keyListener(event: KeyboardEvent) {
    if (event.key === "Escape") {
        closeDropdown();
    }
}

onMount(async () => {
    document.addEventListener("keydown", keyListener);

    let details = document.getElementById(uuid)!;
    details.addEventListener("toggle", function() {
        isOpen = details.hasAttribute("open");
        if (isOpen) {
            closeAllPopups()
        }
    });
});

onDestroy(() => {
    document.removeEventListener("keydown", keyListener);
});

</script>

<details id={uuid} class="dropdown dropdown-start" use:clickOutside={closeDropdown}>
    <summary class={"btn btn-sm my-1 items-center rounded-none " + (!empty ? "btn-ghost" : "btn-active btn-neutral cursor-not-allowed" )} >
        <div class={isOpen || empty ? "" : "tooltip tooltip-bottom"} data-tip={tip}>
            <slot name="summary"></slot>
        </div>
    </summary>
    <div class="dropdown-content z-[1] p-0 shadow bg-base-300 rounded-md my-0 font-normal max-scroll-area-height overflow-y-auto max-h-96"
        bind:clientHeight={contentHeight}>
        <slot name="details"></slot>
    </div>
</details>
