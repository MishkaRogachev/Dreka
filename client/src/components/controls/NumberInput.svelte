<script lang="ts">
import NumberSpinner from "svelte-number-spinner"
import { createEventDispatcher } from "svelte"

import Button from "./Button.svelte"

import { countDecimals } from "$lib/common/formats"

import plusIcon from "$assets/svg/plus.svg"
import minusIcon from "$assets/svg/minus.svg"

export let min: number = 0
export let max: number = 999
export let value: number = 0
export let step: number = 0.001
export let decimals: number = countDecimals(step)
export let precision: number = step

export let style: string = ""

const dispatch = createEventDispatcher()

function change() { dispatch('change', {}) }

</script>

<style>
#container {
    display: flex;
}
</style>

<div id="container" style={style}>
    <Button style="width:20%" icon={minusIcon} flat={true} right_cropped={true} disabled={value <= min}
        on:click={() => { value = Math.max(value - step, min) }}/>
    <NumberSpinner min={min} max={max} step={step} decimals={decimals} precision={precision} editOnClick={true}
        mainStyle="width:60%; height:24px; color:white; font-size:14px; background-color:black; text-align:center; border:2px solid rgba(5, 108, 98, 1.0); border-radius:0px;"
        focusStyle="border: 2px solid rgba(0, 218, 223, 1.0); outline: none;"
        bind:value={value} on:change={change}/>
    <Button style="width:20%" icon={plusIcon} flat={true} left_cropped={true} disabled={value >= max}
        on:click={() => { value = Math.min(value + step, max) }}/>
</div>