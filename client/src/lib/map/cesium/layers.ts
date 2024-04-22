import type { ImageryLayer, MapLayers } from '$lib/interfaces/map';

import * as Cesium from 'cesium';

const defaultLayers = [{
    name: "ArcGIS World Imagery",
    source: "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}",
    opacity: 1.0,
    visibility: true,
    index: 3
}, {
    name: "ArcGIS World Street Maps",
    source: "https://services.arcgisonline.com/ArcGIS/rest/services/World_Street_Map/MapServer/tile/{z}/{y}/{x}",
    opacity: 1.0,
    visibility: false,
    index: 2
}, {
    name: "Google",
    source: "http://mt0.google.com/vt/lyrs=y&hl=en&x={x}&y={y}&z={z}",
    opacity: 1.0,
    visibility: false,
    index: 1
}];

export class MapLayersCesium implements MapLayers {
    constructor(cesium: Cesium.Viewer) {
        this.cesium = cesium;
        this.imageryLayers = new Map();

        // Clear default imagery layers
        this.cesium.imageryLayers.removeAll();
    }

    async resetImageryLayers() {
        this.addImageryLayers(defaultLayers);
    }

    async addImageryLayers(layers: ImageryLayer[]) {
        layers.sort((a, b) => a.index - b.index);
        layers.forEach(async layer => await this.addImageryLayer(layer));
    }

    async addImageryLayer(layer: ImageryLayer) {
        const provider = new Cesium.UrlTemplateImageryProvider({
            credit: layer.name,
            url: layer.source,
        })

        const layerItem = this.cesium.imageryLayers.addImageryProvider(provider);
        layerItem.alpha = layer.opacity;
        layerItem.show = layer.visibility;
        this.imageryLayers.set(layer, layerItem);
    }

    async updateImageryLayer(layer: ImageryLayer) {
        const layerItem = this.imageryLayers.get(layer)!;
        layerItem.alpha = layer.opacity;
        layerItem.show = layer.visibility;
    }

    async removeImageryLayer(layer: ImageryLayer) {
        const layerItem = this.imageryLayers.get(layer)!;
        this.cesium.imageryLayers.remove(layerItem);
        this.imageryLayers.delete(layer);
    }

    async raiseImageryLayer(layer: ImageryLayer) {
        const layerItem = this.imageryLayers.get(layer)!;
        this.cesium.imageryLayers.raise(layerItem);
    }

    async lowerImageryLayer(layer: ImageryLayer) {
        const layerItem = this.imageryLayers.get(layer)!;
        this.cesium.imageryLayers.lower(layerItem);
    }

    allImageryLayers(): Array<ImageryLayer> {
        let array: Array<ImageryLayer> = [];
        this.imageryLayers.forEach((layerItem, layer) => {
            layer.index = this.cesium.imageryLayers.indexOf(layerItem);
            array.push(layer);
        })

        array.sort((a, b) => a.index - b.index);
        return array;
    }

    private cesium: Cesium.Viewer
    private imageryLayers: Map<ImageryLayer, Cesium.ImageryLayer>
}
