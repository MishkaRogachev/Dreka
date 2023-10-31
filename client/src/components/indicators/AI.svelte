<script lang="ts">
// @ts-ignore
import { Canvas, Layer } from "svelte-canvas"

export let pitch: number = 0
export let roll: number = 0
export let online: boolean = false

// @ts-ignore
$: horizon = ({ context, width, height }) => {
    const x = width / 2
    const y = height / 2
    const radius = height * 0.5

    let sky = context.createLinearGradient(0, 0, 0, -width)
    sky.addColorStop(1.0, online ? "#1565c0" : "#6b6b6b")
    sky.addColorStop(0.0, online ? "#80deea" : "#b5b5b5")

    let grd = context.createLinearGradient(0, 0, 0, radius)
    grd.addColorStop(1.0, online ? "#fb8c00" : "#7d7d7d")
    grd.addColorStop(0.0, online ? "#fdd835" : "#999999")

    // Sky
    context.save()
    context.translate(x, y)

    context.save()
    context.rotate(-roll * Math.PI / 180)

    const pitchR = pitch * Math.PI / 180

    context.fillStyle = sky
    context.beginPath()
    context.arc(0, 0, radius, Math.PI - pitchR, Math.PI * 2 + pitchR, false)
    context.fill()

    // Gound
    context.fillStyle = grd
    context.beginPath()
    context.arc(0, 0, radius, pitchR, Math.PI - pitchR, false)
    context.fill()

    // Horizon line
    const horizonY = Math.sin(pitchR) * y
    const radiusR = radius * Math.cos(pitchR)

    context.lineWidth = 2
    context.strokeStyle = "white"
    context.beginPath()
    context.moveTo(-radiusR, horizonY)
    context.lineTo(radiusR, horizonY)
    context.stroke()

    context.restore()

    // Plane mark
    context.lineWidth = 2
    context.strokeStyle = "black"

    context.beginPath()
    context.moveTo(-radius / 1.5, 0)
    context.lineTo(-radius / 3, 0)
    context.lineTo(-radius / 6, radius / 6)
    context.lineTo(0, 0)
    context.lineTo(radius / 6, radius / 6)
    context.lineTo(radius / 3, 0)
    context.lineTo(radius / 1.5, 0)

    context.moveTo(0, 0)
    context.arc(0, Math.cos(pitchR), context.lineWidth, 0, 2 * Math.PI, false)
    context.stroke()

    context.restore()
};

</script>

<Canvas width={96} height={128}>
    <Layer render={horizon} />
</Canvas>
