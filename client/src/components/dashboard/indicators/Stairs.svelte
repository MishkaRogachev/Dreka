<script lang="ts">
import { cssColorToHex } from "$lib/common/formats";
import { Canvas, Layer } from "svelte-canvas"

export let value: number = 0
export let maxValue: number = 100

export let canvas_class: string = ""
export let steps = 5
export let stepMargin = 2

// @ts-ignore
$: stairs = ({ context, width, height }) => {
    const computedStyles = getComputedStyle(document.querySelector(':root')!);
    const bgColor = cssColorToHex(computedStyles.getPropertyValue('--n'));
    const activeColor = cssColorToHex(computedStyles.getPropertyValue('--ac'));

    const stepSizeX = width / steps - stepMargin;
    const stepSizeY = height / steps;
    const stepValue = Math.floor(value / maxValue * steps)

    for (let i = 0; i < steps; i += 1) {
        context.fillStyle = i >= stepValue ? bgColor : activeColor;
        context.fillRect((stepSizeX + stepMargin) * i, (steps - i - 1) * stepSizeY, stepSizeX, height);
    }
}
</script>

<Canvas width={64} height={24} class={canvas_class}>
    <Layer render={stairs} />
</Canvas>

