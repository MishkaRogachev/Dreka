<script lang="ts">
import { cssColorToHex } from "$lib/common/formats";
import { Canvas, Layer } from "svelte-canvas"

export let value: number = 0;
export let minValue: number = 0;
export let maxValue: number = 100;

export let canvas_class: string = "";
export let width = 4;
export let height = 85;
export let color: string | undefined = undefined;

// @ts-ignore
$: bar = ({ context, width, height }) => {
    const computedStyles = getComputedStyle(document.querySelector(':root')!);
    const bgColor = cssColorToHex(computedStyles.getPropertyValue('--n'));
    const activeColor = color ? color : cssColorToHex(computedStyles.getPropertyValue('--su'));

    const start = height / (maxValue - minValue) * minValue;
    const end = -height / (maxValue - minValue) * value;

    context.save();
    context.translate(0, height);
    context.fillStyle = bgColor;
    context.fillRect(0, 0, width, height);

    context.fillStyle = activeColor;
    context.fillRect(0, start, width, end);
    context.restore();
}
</script>

<Canvas width={width} height={height} class={canvas_class}>
    <Layer render={bar} />
</Canvas>

