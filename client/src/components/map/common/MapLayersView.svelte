
<script lang="ts">
import MenuSurface from '@smui/menu-surface';
import Checkbox from '@smui/checkbox';
import Button from '@smui/button';
import DataTable, { Head, Body, Row, Cell } from '@smui/data-table';
import { Text } from '@smui/list';

import type { MapLayers, ImageryLayer } from "$lib/interfaces/map";

import upIcon from "$assets/svg/up.svg?raw"
import downIcon from "$assets/svg/down.svg?raw"

export let layers: MapLayers

let imageryLayers = layers.imageryLayers();

let surface: MenuSurface

export function setOpen(open: boolean) { surface.setOpen(open); }
export function isOpened() { return surface.isOpen(); }

async function switchLayerVisibility(imageryLayer: ImageryLayer) {
    imageryLayer.visibility = !imageryLayer.visibility;
    layers.updateImageryLayer(imageryLayer);
    imageryLayers = layers.imageryLayers();
}

async function switchLayerOpacity(imageryLayer: ImageryLayer) {
    if (imageryLayer.opacity < 0.25 || imageryLayer.opacity >= 1.0) {
        imageryLayer.opacity = 0.25;
    } else if (imageryLayer.opacity < 0.50) {
        imageryLayer.opacity = 0.50;
    } else if (imageryLayer.opacity < 0.75) {
        imageryLayer.opacity = 0.75;
    } else {
        imageryLayer.opacity = 1.0;
    }
    layers.updateImageryLayer(imageryLayer);
    imageryLayers = layers.imageryLayers();
}

async function raiseLayer(imageryLayer: ImageryLayer) {
    layers.raiseImageryLayer(imageryLayer);
    imageryLayers = layers.imageryLayers();
}

async function lowerLayer(imageryLayer: ImageryLayer) {
    layers.lowerImageryLayer(imageryLayer);
    imageryLayers = layers.imageryLayers();
}
</script>

<MenuSurface bind:this={surface} anchorCorner="BOTTOM_START" style="width: 400px">
    <DataTable style="width: 100%;">
        <Head>
            <Row>
                <Cell>Visible</Cell>
                <Cell>Layer</Cell>
                <!-- <Cell>Move</Cell> -->
                <Cell>Opacity</Cell>
            </Row>
        </Head>
        <Body>
            {#each imageryLayers as imageryLayer}
            <Row>
                <Cell><Checkbox checked={imageryLayer.visibility} on:click={ () => { switchLayerVisibility(imageryLayer) }} /></Cell>
                <Cell><Text>{imageryLayer.name}</Text></Cell>
                <!-- <Cell>
                    <IconButton disabled={imageryLayer.index < 2} on:click={ () => { raiseLayer(imageryLayer) }} size="mini">
                        {@html downIcon}</IconButton>
                    <IconButton disabled={imageryLayer.index > imageryLayers.length - 1} on:click={ () => { lowerLayer(imageryLayer) }} size="mini">
                        {@html upIcon}</IconButton>
                </Cell> -->
                <Cell><Button on:click={ () => { switchLayerOpacity(imageryLayer) }}>
                    <Text>{Math.round(imageryLayer.opacity * 100) + "%"}</Text>
                </Button></Cell> 
            </Row>
            {/each}
        </Body>
    </DataTable>
</MenuSurface>

<!-- / TODO: Map layers order like here https://sandcastle.cesium.com/index.html?src=Imagery%2520Layers%2520Manipulation.html -->
