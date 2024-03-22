<script lang="ts">
import { type Sensor } from "$bindings/telemetry";

export let sensor: Sensor
</script>

<div class={"kbd " + (sensor.enabled ? sensor.health ? "" : "text-error" : "disabled" )}>
    <!-- TODO: sensor icons -->
    {sensor.name}
</div>