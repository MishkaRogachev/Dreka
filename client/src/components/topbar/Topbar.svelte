<script lang="ts">
import { dashboardVisible } from "$stores/app";
import { i18n } from "$stores/i18n";

import Menu from "./Menu.svelte";
import VehicleSelector from "./VehicleSelector.svelte";
import VehicleSummary from "./VehicleSummary.svelte";

import hideIcon from "$assets/svg/hide_dashboard.svg?raw";
import showIcon from "$assets/svg/show_dashboard.svg?raw";

</script>

<div class="navbar bg-base-300 btn-sm px-1">
    <div class="navbar-start flex items-center">
        <Menu/>
    </div>

    <div class="navbar-end font-bold flex items-center gap-x-2">
        <VehicleSelector/>
        <VehicleSummary/>
        <div class="tooltip tooltip-left" data-tip={ $dashboardVisible ? $i18n.t("Hide dashboard") : $i18n.t("Show dashboard") }>
            <button class="btn btn-sm btn-ghost btn-circle my-1" on:click={() => { $dashboardVisible = !$dashboardVisible }}>
                { @html $dashboardVisible ? hideIcon : showIcon}
            </button>
        </div>
    </div>
</div>