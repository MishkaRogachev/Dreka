<script lang="ts">
export let name: string
export let tooltip: string
export let value: number
export let precision: number = 0

$: parsedValue = value > 10000 ? Math.round(value / 1000) + "k" : value.toFixed(precision)

</script>

<div class="tooltip tooltip-left" data-tip={tooltip}>
    <div class="flex flex-col items-center w-12">
        <a href={null} class="text-sm">{name}</a>
        <span class="countdown font-mono font-bold text-lg">{parsedValue}</span>
    </div>
</div>