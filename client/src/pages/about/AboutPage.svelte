<script lang="ts">
import Label from "$components/controls/Label.svelte";
</script>

<style>
#about {
    width: 100%;
    min-height: 50%;
    max-height: 100%;
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px;
}
</style>

<div id="about" class="panel">
    <Label text="About" />
</div>