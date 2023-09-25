<script lang="ts">
import Overlay from "svelte-overlay"
import Button from "./Button.svelte"

export let text: String  = ""
export let icon = ""
export let disabled = false
export let opacity = true
export let flat = false
export let left_cropped = false
export let right_cropped = false
export let top_cropped = false
export let bottom_cropped = false
export let style = ""

export let position = ""
export let margin: number = 5

export let isOpen = false

export function close() {
    isOpen = false
}

$: marginStyle = positionToMargin() + ": " + margin + "px"

function closeOnEscape(event: any) {
    if (event.key === 'Escape') {
        isOpen = false
    }
}

function positionToMargin() {
    switch (position) {
    case "left-top":
    case "left-center":
    case "left-bottom":
        return "margin-right"
    case "right-top":
    case "right-center":
    case "right-bottom":
        return "margin-left"
    case "bottom-center":
    case "bottom-left":
    case "bottom-right":
        return "margin-top"
    case "top-center":
    case "top-left":
    case "top-right":
    default:
        return "margin-bottom"
    }
}
</script>

<style>
#popup {
    width: max-content;
    padding: 5px;
}
</style>

<Overlay
    onWindowKeyDown={closeOnEscape}
    closeOnClickOutside
    position={position}
    zIndex=666
    style="margin: 0px"
    bind:isOpen={isOpen}>
    <Button
     slot="parent"
     flat={flat}
     disabled={disabled}
     selected={isOpen}
     style={style}
     text={text}
     icon={icon}
     left_cropped={left_cropped}
     right_cropped={right_cropped}
     top_cropped={top_cropped}
     bottom_cropped={bottom_cropped}
     let:toggle on:click={toggle}/>
    <div slot="content" id="popup" class={opacity ? "pane" : "frame"} style={marginStyle}>
        <slot></slot>
    </div>
</Overlay>