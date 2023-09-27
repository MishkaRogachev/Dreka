<script lang="ts">
export let visible: boolean = true
</script>

<style>
#settingsPanel {
    color: white;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    background: transparent;
}
</style>

<div id="settingsPanel" style={visible ? "" : "display: none"}>
    <span>Settings will be here..</span>
</div>

