<script lang="ts">

export let isMenuOpen = false;
export let menuPosition = { x: 0, y: 0 };

let menuWidth: number
let menuHeight: number

const pointerSize: number = 10

export function closeMenu() {
    isMenuOpen = false;
}

</script>

<style>
#menu {
    position: absolute;
    min-width: 128px
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

{#if isMenuOpen}
<div
    id="menu"
    class="border menu bg-base-300 rounded-md"
    style="top: {menuPosition.y - menuHeight - pointerSize}px; left: {menuPosition.x - menuWidth / 2}px;"
    bind:clientWidth={menuWidth}
    bind:clientHeight={menuHeight}>
    <slot></slot>
</div>
<div id="pointer" style="top: {menuPosition.y - pointerSize}px; left: {menuPosition.x - pointerSize / 2}px;"></div>
{/if}
