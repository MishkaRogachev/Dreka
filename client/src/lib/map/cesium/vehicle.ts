import { type VehicleDescription } from '$bindings/vehicles';
import type { Flight } from '$bindings/telemetry';
import { toColorCode } from '$bindings/colors';

import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { ModelEntity, PylonEntity, PathEntity } from "./base-entities"

import * as Cesium from 'cesium';

// @ts-ignore
import fixedWing from "$assets/3d/art_v1.glb"

export class MapVehicleCesium {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {

        this.model = new ModelEntity(cesium);
        this.pylon = new PylonEntity(cesium, 4.0);
        this.path = new PathEntity(cesium, 100);
    }

    done() {
        this.path.done();
        this.pylon.done();
        this.model.done();
    }

    cartesian(): Cesium.Cartesian3 {
        return this.model.cartesian();
    }

    updateFromDescription(vehicle: VehicleDescription) {
        this.model.addLabel(vehicle.name);
        this.model.setUri(fixedWing); // TODO: get from type

        const color = Cesium.Color.fromCssColorString(toColorCode(vehicle.color));
        this.model.setBaseColor(color);
        this.pylon.setBaseColor(color);
        this.path.setBaseColor(color);
    }

    updateFromFlight(flight: Flight) {
        const cartesian = flight.position.longitude === 0 && flight.position.latitude === 0
            ? Cesium.Cartesian3.ZERO
            : Cesium.Cartesian3.fromDegrees(flight.position.longitude, flight.position.latitude, flight.position.altitude);

        if (cartesian !== Cesium.Cartesian3.ZERO && !this.cartesian().equals(cartesian)) {
            this.model.setCartesian(cartesian);
            this.pylon.setCartesian(cartesian);
            this.path.addCartesian(cartesian);
        }
        this.model.setHpr(flight.yaw, flight.pitch, flight.roll);
    }

    setSelected(selected: boolean) {
        this.model.setSilhouetteColor(selected ? Cesium.Color.WHITE : Cesium.Color.GRAY);
        this.path.setVisible(selected);
    }

    private path: PathEntity
    private model: ModelEntity
    private pylon: PylonEntity
}
