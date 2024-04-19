<script lang="ts">
export let name: string
export let tooltip: string
export let value: number
export let precision: number = 0
</script>

<div class="tooltip tooltip-left" data-tip={tooltip}>
    <div class="flex flex-col items-center w-12">
        <a href={null} class="text-sm">{name}</a>
        <span class="countdown font-mono font-bold text-lg">{value.toFixed(precision)}</span>
    </div>
</div>