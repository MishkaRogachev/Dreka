<script lang="ts">
import { Pages, currentPage } from '$stores/app';

import burgerIcon from "$assets/svg/burger.svg?raw";
import fleetIcon from "$assets/svg/fleet.svg?raw";
import connectIcon from "$assets/svg/connect.svg?raw";
import aboutIcon from "$assets/svg/about.svg?raw";
</script>

<div class="dropdown">
    <label tabindex="0" class="btn btn-ghost btn-sx rounded-btn">{@html burgerIcon}</label>
    <ul tabindex="0" class="dropdown-content menu z-[1] p-2 shadow bg-base-100 rounded-box my-1">
        <li class="btn-wide"><a on:click={() => {$currentPage = Pages.Flight}}>{@html fleetIcon} Flight</a></li>
        <li class="btn-wide"><a on:click={() => {$currentPage = Pages.Communication}}>{@html connectIcon} Communication</a></li> 
        <li class="btn-wide"><a on:click={() => {$currentPage = Pages.About}}>{@html aboutIcon} About</a></li> 
    </ul>
</div>
