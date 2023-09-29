<script lang="ts">
import { createEventDispatcher } from "svelte"

export let text: String = "";
export let icon = "";
export let disabled = false;
export let selected = false;
export let flat = false;
export let left_cropped = false;
export let right_cropped = false;
export let top_cropped = false;
export let bottom_cropped = false
export let style: string = "";
export let type: string = "normal";

$: iconClass = disabled ? "button-icon icon-disabled " : selected ? "button-icon icon-selected" : "button-icon";

function getButtonClass(flat: boolean, left_cropped: boolean, right_cropped: boolean, top_cropped: boolean, bottom_cropped: boolean,
                        type: string) {
    let arr = [];

    if (flat)
        arr.push("flat");
    if (left_cropped)
        arr.push("left-cropped");
    if (right_cropped)
        arr.push("right-cropped");
    if (top_cropped)
        arr.push("top-cropped");
    if (bottom_cropped)
        arr.push("bottom-cropped");

    switch (type){
    case "danger":
        arr.push("button-danger");
        break
    case "warning":
        arr.push("button-warning");
        break
    case "normal":
    default:
        arr.push("button-normal");
        break
    }

    return arr.join(" ");
}

const dispatch = createEventDispatcher()

function click() { dispatch('click', {}); }
function mousedown() { dispatch('mousedown', {}); }
function mouseup() { dispatch('mouseup', {}); }
function mouseleave() { dispatch('mouseleave', {}); }

</script>

<style>
.button-danger:active:not([disabled]) {
    background: rgba(144, 11, 6, 0.85);
}
.button-warning:active:not([disabled]) {
    background: rgba(144, 89, 6, 0.85);
}
.button-normal:active:not([disabled]) {
    background: rgba(6, 144, 130, 0.85);
}

.button-danger:checked {
    background: rgba(108, 5, 5, 0.85);
}
.button-warning:checked {
    background: rgba(108, 77, 5, 0.85);
}
.button-normal:checked {
    background: rgba(5, 108, 98, 0.85);
}

.button-danger:focus {
    border: 1px solid rgb(223, 26, 0);
}
.button-warning:focus {
    border: 1px solid rgb(223, 171, 0);
}
.button-normal:focus {
    border: 1px solid rgba(0, 218, 223, 1.0);
}

.button-txt {
    display: inline-block;
    width: 100%;
    text-align: center;
}
.button-txt-icon {
    display: inline-block;
    width: 80%;
    text-align: center;
}
#btn-icon {
    margin-left: auto;
    margin-right: auto;
    display: block !important;
}
</style>

<button class={getButtonClass(flat, left_cropped, right_cropped, top_cropped, bottom_cropped, type)} disabled={disabled} style={style}
    on:click={click} on:mousedown={mousedown} on:mouseup={mouseup} on:mouseleave={mouseleave}>
    {#if icon !== ""}
    <img id={text === "" ? "btn-icon" : ""} class={iconClass} src={icon} alt="-"/>
    {/if}

    {#if text !== ""}
    <div class={icon !== "" ? "button-txt-icon" : "button-txt"} style={disabled ? "color: rgb(59, 55, 55);" : ""}><b>{text}</b></div>
    {/if}
    <slot/>
</button>

