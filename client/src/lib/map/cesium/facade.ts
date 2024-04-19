import * as Cesium from 'cesium';

import type { MapFacade } from "$lib/interfaces/map";

import { MapViewportCesium } from '$lib/map/cesium/viewport';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { MapRulerCesium } from '$lib/map/cesium/ruler';
import { MapGraticuleCesium } from '$lib/map/cesium/graticule';
import { MapLayersCesium } from '$lib/map/cesium/layers';
import { MapVehiclesCesium } from '$lib/map/cesium/vehicles';
import { MapMissionsCesium } from '$lib/map/cesium/missions';
import type { Geodetic } from '$bindings/spatial';

export class MapFacadeCesium implements MapFacade {
    constructor(container: string) {
        this.cesium = new Cesium.Viewer(
            container, {
                orderIndependentTranslucency: false,
                timeline: false,
                geocoder: false,
                selectionIndicator: false,
                infoBox: false,
                scene3DOnly: true,
                shouldAnimate: true,
                baseLayerPicker: false,
            });
            this.cesium.resolutionScale = window.devicePixelRatio;

        this.viewport = new MapViewportCesium(this.cesium);
        this.interaction = new MapInteractionCesium(this.cesium);
        this.ruler = new MapRulerCesium(this.cesium, this.interaction);
        this.graticule = new MapGraticuleCesium(this.cesium);
        this.layers = new MapLayersCesium(this.cesium);

        this.vehicles = new MapVehiclesCesium(this.cesium, this.interaction);
        this.missions = new MapMissionsCesium(this.cesium, this.interaction);
    }

    async initTerrain() {
        // TODO: terrain layers
        this.cesium.terrainProvider = await Cesium.createWorldTerrainAsync({
            requestVertexNormals: true,
            requestWaterMask: true
        });
    }

    done() {
        this.vehicles.done();
        this.missions.done();
    }

    calcDistance(from: Geodetic, to: Geodetic): number {
        if (from === to || !from || !to) return 0;
        return Cesium.Cartesian3.distance(
            Cesium.Cartesian3.fromDegrees(from.longitude, from.latitude, from.altitude),
            Cesium.Cartesian3.fromDegrees(to.longitude, to.latitude, to.altitude)
        );
    }

    cesium: Cesium.Viewer

    viewport: MapViewportCesium;
    interaction: MapInteractionCesium;
    ruler: MapRulerCesium;
    graticule: MapGraticuleCesium;
    layers: MapLayersCesium;
    vehicles: MapVehiclesCesium;
    missions: MapMissionsCesium;
}
