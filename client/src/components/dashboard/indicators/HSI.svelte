<script lang="ts">
import { cssColorToHex } from "$lib/common/formats";
import { Canvas, Layer } from "svelte-canvas"

export let heading: number = 0
export let course: number = 0
export let courseEnabled: boolean = false

export let tickFactor = 30
export let scalesRatio = 0.075

export let canvas_class: string = ""

// @ts-ignore
$: compas = ({ context, width, height }) => {
    const computedStyles = getComputedStyle(document.querySelector(':root')!);
    const currentColor = cssColorToHex(computedStyles.getPropertyValue('--bc'));

    const x = width / 2;
    const y = height / 2;

    const scalesOffset = width * scalesRatio;
    const minorTickOffset = 0.33 * scalesOffset;
    const majorTickOffset = 0.66 * scalesOffset;
    const textOffset = 2.5 * scalesOffset;

    context.strokeStyle = currentColor;
    context.fillStyle = currentColor;
    context.textBaseline = 'middle';
    context.textAlign = 'center';

    context.save();
    context.translate(x, y);
    context.save();

    // Scales
    for (let i = 0; i <= 360; i += tickFactor) {
        context.lineWidth = i % 30 ? 1 : 2;
        context.beginPath();
        context.save();
        context.rotate((i - heading) * Math.PI / 180);
        context.translate(0, -y);

        if (!(i % 90)) {
            context.moveTo(0, textOffset);
            context.lineTo(0, textOffset + majorTickOffset);

            context.save();
            context.translate(0, textOffset / 2);
            context.rotate((-i + heading) * Math.PI / 180);

            if (i == 0) context.fillText("N", 0, 2);
            else if (i == 90) context.fillText("E", 0, 2);
            else if (i == 180) context.fillText("S", 0, 2);
            else if (i == 270) context.fillText("W", 0, 2);
            context.restore();
        } else {
            context.moveTo(0, textOffset + majorTickOffset - minorTickOffset);
            context.lineTo(0, textOffset + majorTickOffset);
        }
        context.restore();
        context.stroke();
    }
    context.restore();

    // Course Mark
    if (courseEnabled) {
        const tY = textOffset + majorTickOffset - y;
        context.fillStyle = currentColor;
        context.save();
        context.rotate((course - heading) * Math.PI / 180);
        context.beginPath();
        context.moveTo(0, tY);
        context.lineTo(majorTickOffset, tY + majorTickOffset);
        context.lineTo(0, tY + minorTickOffset);
        context.lineTo(-majorTickOffset, tY + majorTickOffset);
        context.closePath();
        context.fill();
        context.restore();
    }
    context.restore();
}
</script>

<Canvas width={112} height={112} class={canvas_class}>
    <Layer render={compas} />
</Canvas>

