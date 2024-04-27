<script lang="ts">
import { onMount, onDestroy } from 'svelte';
import { clickOutside } from '$lib/common/click-outside';

export let tip: string;

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
    });
});

onDestroy(() => {
    document.removeEventListener("keydown", keyListener);
});

</script>

<details id={uuid} class="dropdown dropdown-start" use:clickOutside={closeDropdown}>
    <summary class="select select-ghost select-xs my-2 items-center">
        <div class={isOpen ? "" : "tooltip tooltip-bottom"} data-tip={tip}>
            <slot name="summary"></slot>
        </div>
    </summary>
    <div class="dropdown-content z-[1] p-0 shadow bg-base-300 rounded-md my-0 font-normal max-scroll-area-height overflow-y-auto max-h-96">
        <slot name="details"></slot>
    </div>
</details>
