<script lang="ts">
export let name: string
export let tooltip: string
export let value: number | undefined
export let precision: number = 0

$: parsedValue = !!value ? value > 10000 ? Math.round(value / 1000) + "k" : value.toFixed(precision) : "0"

function toTextColor(value: number | undefined) {
    if (value === undefined) {
        return "text-neutral";
    }
    return ""
}

</script>

<div class="tooltip tooltip-left" data-tip={tooltip}>
    <div class={"flex flex-col items-center w-12 " + toTextColor(value)}>
        <a href={null} class="text-sm">{name}</a>
        <span class="font-mono font-bold text-lg">{parsedValue}</span>
    </div>
</div>
