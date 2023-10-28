<script lang="ts">

import { type MavlinkProtocolId } from "$bindings/vehicles";

import { allMavlinkIds, occupiedMavlinkIds } from "$stores/vehicles";
import { i18n } from "$stores/i18n";

export let disabled: boolean
export let protocol_id: MavlinkProtocolId

</script>

<!-- MAV ID (SYSTEM ID) EDIT -->
<h1 class="font-medium my-2 w-full">{ $i18n.t("MAVLink System ID") }</h1>
<select class="select w-full" disabled={disabled} bind:value={protocol_id.mav_id} >
    {#each allMavlinkIds as mavId}
    <option value={mavId} disabled={$occupiedMavlinkIds.includes(mavId)}>
        { mavId + (protocol_id.mav_id !== mavId && $occupiedMavlinkIds.includes(mavId) ? " (occupied)" : "")}
    </option>
    {/each}
</select>
