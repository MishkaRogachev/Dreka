<script lang="ts">
export let style: string = "";
export let state: string = "off";
</script>

<style>
#led {
    width: 8px;
    height: 8px;
    border-radius: 8px;
    margin: 8px 8px;
}

.on {
    box-shadow: 0 0 2px 2px teal;
    background-color: turquoise;
}

.error {
    box-shadow: 0 0 2px 2px rgb(104, 4, 40);
    background-color: rgb(224, 64, 72);
}

.off {
    box-shadow: 0 0 0px 0px black;
    background-color: rgb(88, 88, 88)
}

</style>

<div id="led" class={state} style={style}/>
