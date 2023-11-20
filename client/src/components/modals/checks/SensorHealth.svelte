<script lang="ts">
import { type SensorData } from "$bindings/telemetry";

export let sensor: SensorData
</script>

<div class={"kbd " + (sensor.enabled ? sensor.health ? "" : "text-error" : "disabled" )}>
    <!-- TODO: sensor icons -->
    {sensor.name}
</div>