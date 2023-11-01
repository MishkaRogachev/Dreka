<script lang="ts">
export let name: string = ""
export let value: number = 0
export let precision: number = 0
</script>

<div class="flex flex-col items-center">
    <a class="text-sm">{name}</a>
    <span class="countdown font-mono text-xl">{value.toFixed(precision)}</span>
</div>
