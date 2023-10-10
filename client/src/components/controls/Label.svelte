<script lang="ts">

export let text: String = "Text"
export let style: string = ""

</script>

<style>
#label {
    display: inline-flex;
    text-align: center;
    padding: 0px;
    vertical-align: middle;
    line-height: 24px; /* <-- adjust this */
}

</style>

<div id="label" class="noselect" style={style}>
    {text}
</div>
