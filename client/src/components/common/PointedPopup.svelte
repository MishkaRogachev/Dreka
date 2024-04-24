<script lang="ts">

export let isPopupOpen = false;
export let popupPosition = { x: 0, y: 0 };
export let noInput = false

let popupWidth: number
let popupHeight: number

const pointerSize: number = 10

</script>

<style>
#popup {
    position: absolute;
}
#pointer {
    position: absolute;
    width: 0;
    height: 0;
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;
    border-top: 10px solid rgba(0, 0, 0, 0.85);
}
</style>

{#if isPopupOpen}
<div
    id="popup"
    class={"border bg-base-300 rounded-md " + (noInput ? "pointer-events-none" : "")}
    style="top: {popupPosition.y - popupHeight - pointerSize}px; left: {popupPosition.x - popupWidth / 2}px;"
    bind:clientWidth={popupWidth}
    bind:clientHeight={popupHeight}>
    <slot></slot>
</div>
<div id="pointer" class="pointer-events-none" style="top: {popupPosition.y - pointerSize}px; left: {popupPosition.x - pointerSize / 2}px;"></div>
{/if}
