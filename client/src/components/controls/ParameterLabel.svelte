<script lang="ts">
import { roundPrecision } from "$lib/common/formats"

export let name: string
export let value: number
export let precision: number = 0
</script>

<style>
h3 {
    margin-top: 0.33em;
}
</style>

<div>
    {name}
    <h3>{roundPrecision(value, precision)}</h3>
</div>
