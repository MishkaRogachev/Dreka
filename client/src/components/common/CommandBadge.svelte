<script lang="ts">
import type { CommandState } from "$bindings/commands";

export let baseClass: string = "badge-neutral"
export let state: CommandState | undefined = undefined;

const RESTORE_DELAY = 500;

let value: number = 0;
let stateClass: string = baseClass

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
    } else {
        setTimeout(() => {
            stateClass = baseClass;
            value = 0;
        }, RESTORE_DELAY)
    }
}

$: onStateChanged(state);

</script>

<div class={"badge badge-xs " + stateClass}><slot></slot></div>
