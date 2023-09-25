<script lang="ts">

export let text: String = "Text"
export let style: string = ""

</script>

<style>
#label {
    display: inline-flex;
    text-align: center;
    margin-top: 6px;
    padding: 0px;
}
</style>

<div id="label" style={style}>
    {text}
</div>
