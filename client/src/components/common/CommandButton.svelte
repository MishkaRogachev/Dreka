<script lang="ts">
import { createEventDispatcher } from "svelte"

import type { CommandState } from "$bindings/commands";

import CommandBadge from "$components/common/CommandBadge.svelte";

export let btnClass: string = "btn"
export let disabled = false
export let state: CommandState | undefined = undefined;

const dispatch = createEventDispatcher()

function onclick() {
    if (state && (state.Initial || state.Sent || state.InProgress)) {
        dispatch('cancel', {})
    } else if (!state) {
        dispatch('execute', {})
    }
}

function mousedown() { dispatch('mousedown', {}) }
function mouseup() { dispatch('mouseup', {}) }
function mouseleave() { dispatch('mouseleave', {}) }

</script>

<button class={btnClass} disabled={disabled}
    on:click={onclick} on:mousedown={mousedown} on:mouseup={mouseup} on:mouseleave={mouseleave}>
    <div class="grow">
        <slot></slot>
    </div>
    <CommandBadge state={state}></CommandBadge>
</button>
