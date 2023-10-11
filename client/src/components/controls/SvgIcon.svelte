<script lang="ts">
export let src: string = "";
export let rotation: number = 0.0;
export let size: number = 16
</script>

<style>
img {
    padding: 8px;
}
</style>

<img src={src} alt="Compas" style="width: {size}px; height: {size}px; transform:rotate({rotation}deg);" />