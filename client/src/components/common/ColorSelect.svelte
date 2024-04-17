<script lang="ts">
import { clickOutside } from '$lib/common/click-outside';

import { toColorCode } from '$bindings/colors';
import { i18n } from '$stores/i18n';

function closeDropdown() {
    document.getElementById("colorSelectDropdown")?.removeAttribute("open");
}

export let colors: Array<string>
export let currentColor: string

</script>

<details id="colorSelectDropdown" class="dropdown w-full" use:clickOutside={closeDropdown}>
    <summary class="select w-full">
        <div class="flex gap-x-2 items-center grow">
            <a href={null} class="grow">{ $i18n.t(currentColor) }</a>
            <kbd class="kbd kbd-sm" style="background-color: {toColorCode(currentColor)}"/>
        </div>
    </summary>
    <ul class="dropdown-content w-full menu z-[1] p-0 shadow bg-base-300 rounded-md my-0 h-20">
    {#each colors as color}
        <li class={"flex " + (currentColor === color ? "font-bold" : "")}
            on:click = {() => { currentColor = color; closeDropdown(); }}>
            <div class="flex gap-x-2 items-center grow">
                <a href={null} class="grow">{ $i18n.t(color) }</a>
                <kbd class="kbd kbd-sm" style="background-color: {toColorCode(color)}"/>
            </div>
        </li>
    {/each}
    </ul>
</details>
