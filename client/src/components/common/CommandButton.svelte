<script lang="ts">
import { createEventDispatcher } from "svelte"

import type { CommandState } from "$bindings/commands";

export let btnClass: string = "btn"
export let progressClass: string = ""
export let disabled = false
export let state: CommandState | undefined = undefined;

let value: number = 0;
let stateClass: string = progressClass

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

function onStateChanged(state: CommandState | undefined) {
    if (state) {
        if (state.Initial || state.Sent || state.InProgress) {
            stateClass = "progress-warning";
            value = 100;
        } else if (state.Accepted) {
            stateClass = "progress-success";
            value = 100;
        } else if (state.Denied || state.Failed || state.Rejected || state.Unsupported) {
            stateClass = "progress-error";
            value = 100;
        } else { // Canceled or others
            stateClass = progressClass;
            value = 0;
        }
    } else {
        setTimeout(() => {
            stateClass = progressClass;
            value = 0;
        }, 500)
    }
}

$: onStateChanged(state);

</script>

<button class={btnClass} disabled={disabled}
    on:click={onclick} on:mousedown={mousedown} on:mouseup={mouseup} on:mouseleave={mouseleave}>
    <slot></slot>
    <progress class={"progress w-full" + " " + stateClass} value="{value}" max="100"></progress>
</button>
