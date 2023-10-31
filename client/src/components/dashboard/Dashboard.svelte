<script lang="ts">
import { selectedVehicleTelemetry } from "$stores/telemetry";

export let visible: boolean = true

$: telemetry = $selectedVehicleTelemetry

</script>