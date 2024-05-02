<script lang="ts">
import { createEventDispatcher } from "svelte"

import type { CommandState } from "$bindings/commands";

export let baseClass: string = "badge-neutral"
export let state: CommandState | undefined = undefined;

const RESTORE_DELAY = 500;

let value: number = 0;
let stateClass: string = baseClass
let oldState: CommandState | undefined = undefined;

const dispatch = createEventDispatcher();

function onStateChanged(state: CommandState | undefined) {
    if (state) {
        if (state.Initial || state.Sent || state.InProgress) {
            stateClass = "badge-warning";
            value = 100;
        } else if (state.Accepted) {
            stateClass = "badge-success";
            value = 100;
        } else if (state.Denied || state.Failed || state.Rejected || state.Unsupported) {
            stateClass = "badge-error";
            value = 100;
        } else { // Canceled or others
            stateClass = baseClass;
            value = 0;
        }
        oldState = state;
    } else if (oldState) {
        setTimeout(() => {
            if (oldState!.Accepted) {
                dispatch("succeeded", {});
            }
            stateClass = baseClass;
            value = 0;
        }, RESTORE_DELAY);
    }
}

$: onStateChanged(state);

</script>

<div class={"badge badge-xs " + stateClass}><slot></slot></div>
