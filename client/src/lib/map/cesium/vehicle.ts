import { GeodeticFrame, type Geodetic } from '$bindings/spatial';
import { type VehicleDescription } from '$bindings/vehicles';
import type { Flight, Navigation } from '$bindings/telemetry';
import { toColorCode } from '$bindings/colors';

import { MapVehicleEvent, type MapVehicle } from '$lib/interfaces/map';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { ModelEntity, PylonEntity, PathEntity, BillboardEntity } from "$lib/map/cesium/base-entities"
import { cartesianFromGeodetic, geodeticFromCartesian } from '$lib/map/cesium/utils';

import * as Cesium from 'cesium';

import homeIcon from "$assets/svg/home.svg";
// @ts-ignore
import fixedWing from "$assets/3d/art_v1.glb"

export class MapVehicleCesium implements MapVehicle {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.listeners = new Map();
        this.interaction = interaction;

        // TODO: click listener to activate vehicle's tooltip
        this.model = new ModelEntity(cesium);
        this.pylon = new PylonEntity(cesium, 4.0);
        this.path = new PathEntity(cesium, 100);

        this.home = new BillboardEntity(cesium);
        this.home.setIcon(homeIcon);
        this.home.setDraggable(true);
        this.home.subscribeDragging((cartesian: Cesium.Cartesian3) => { this.onHomeDragging(cartesian) });
        this.home.subscribeDragged((cartesian: Cesium.Cartesian3) => { this.onHomeDragged(cartesian) });
        this.interaction.addInteractable(this.home);

        this.homePylon = new PylonEntity(cesium, 4.0);
    }

    done() {
        this.interaction.removeInteractable(this.home);
        this.homePylon.done();
        this.home.done();

        this.path.done();
        this.pylon.done();
        this.model.done();
    }

    cartesian(): Cesium.Cartesian3 {
        return this.model.cartesian();
    }

    onHomeDragging(cartesian: Cesium.Cartesian3) {
        this.homePylon.setCartesian(cartesian);
    }

    onHomeDragged(cartesian: Cesium.Cartesian3) {
        const geodetic = geodeticFromCartesian(cartesian, GeodeticFrame.Wgs84AboveSeaLevel, 0);
        if (geodetic) {
            this.invoke(MapVehicleEvent.HomeChanged, geodetic);
        }
    }

    updateFromDescription(vehicle: VehicleDescription) {
        this.model.setUri(fixedWing); // TODO: get from type

        const color = Cesium.Color.fromCssColorString(toColorCode(vehicle.color));
        this.model.setBaseColor(color);
        this.pylon.setBaseColor(color);
        this.path.setBaseColor(color);
    }

    updateFromFlight(flight: Flight) {
        this.model.setHpr(flight.yaw, flight.pitch, flight.roll);
    }

    updateFromNavigation(navigation: Navigation) {
        const cartesian = cartesianFromGeodetic(navigation.position, navigation.home_position.altitude);
        this.model.setCartesian(cartesian);
        this.pylon.setCartesian(cartesian);

        if (cartesian !== Cesium.Cartesian3.ZERO && !this.cartesian().equals(cartesian)) {
            this.path.addCartesian(cartesian);
        }

        const homeCartesian = cartesianFromGeodetic(navigation.home_position, 0);
        this.home.setCartesian(homeCartesian);
        this.homePylon.setCartesian(homeCartesian);
    }

    setSelected(selected: boolean) {
        this.model.setSilhouetteColor(selected ? Cesium.Color.WHITE : Cesium.Color.GRAY);
        this.path.setVisible(selected);
    }

    subscribe(event: MapVehicleEvent, listener: (position: Geodetic) => void) {
        this.listeners.set(event, listener);
    }

    invoke(event: MapVehicleEvent, position: Geodetic) {
        let cb = this.listeners.get(event);
        if (cb) cb(position);
    }

    private interaction: MapInteractionCesium;

    private path: PathEntity
    private model: ModelEntity
    private pylon: PylonEntity

    private home: BillboardEntity
    private homePylon: PylonEntity

    private listeners: Map<MapVehicleEvent, (position: Geodetic) => void>
}
