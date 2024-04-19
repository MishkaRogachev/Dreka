import { GeodeticFrame, type Geodetic } from '$bindings/spatial';
import { type VehicleDescription } from '$bindings/vehicles';
import type { Flight, Navigation } from '$bindings/telemetry';
import { toColorCode } from '$bindings/colors';

import { MapVehiclesEvent, type MapVehicle, type MapVehicles, type MapVehiclesEventListener } from '$lib/interfaces/map';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { ModelEntity, PylonEntity, PathEntity, BillboardEntity } from "$lib/map/cesium/base-entities"
import { cartesianFromGeodetic, geodeticFromCartesian } from '$lib/map/cesium/utils';

import * as Cesium from 'cesium';

import homeIcon from "$assets/svg/home.svg";
// @ts-ignore
import fixedWing from "$assets/3d/art_v1.glb"

export class MapVehicleCesium implements MapVehicle {
    constructor(vehicleId: string, parent: MapVehiclesCesium) {
        this.vehicleId = vehicleId;
        this.parent = parent;

        // TODO: click listener to activate vehicle's tooltip
        this.model = new ModelEntity(parent.cesium);
        this.pylon = new PylonEntity(parent.cesium, 4.0);
        this.path = new PathEntity(parent.cesium, 100);

        this.home = new BillboardEntity(parent.cesium);
        this.home.setIcon(homeIcon);
        this.home.setDraggable(true);
        this.home.subscribeDragging((cartesian: Cesium.Cartesian3) => { this.onHomeDragging(cartesian) });
        this.home.subscribeDragged((cartesian: Cesium.Cartesian3) => { this.onHomeDragged(cartesian) });
        parent.interaction.addInteractable(this.home);

        this.homePylon = new PylonEntity(parent.cesium, 4.0);
    }

    done() {
        this.parent.interaction.removeInteractable(this.home);
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
            this.parent.invoke(MapVehiclesEvent.HomeChanged, this.vehicleId, geodetic);
        }
    }

    centerOnMap() {
        this.model.centerOnMap();
    }

    setTracking(tracking: boolean) {
        this.model.setTracking(tracking);
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

        if (cartesian !== Cesium.Cartesian3.ZERO && !this.cartesian().equals(cartesian)) {
            this.path.addCartesian(cartesian);
        }
        this.model.setCartesian(cartesian);
        this.pylon.setCartesian(cartesian);

        const homeCartesian = cartesianFromGeodetic(navigation.home_position, 0);
        this.home.setCartesian(homeCartesian);
        this.homePylon.setCartesian(homeCartesian);
    }

    setSelected(selected: boolean) {
        this.model.setSilhouetteColor(selected ? Cesium.Color.WHITE : Cesium.Color.GRAY);
        this.path.setVisible(selected);
    }

    private vehicleId: string;
    private parent: MapVehiclesCesium;

    private path: PathEntity
    private model: ModelEntity
    private pylon: PylonEntity

    private home: BillboardEntity
    private homePylon: PylonEntity
}

export class MapVehiclesCesium implements MapVehicles {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.cesium = cesium;
        this.interaction = interaction;

        this.selectedVehicleId = "";
        this.vehicles = new Map();
        this.listeners = new Map();
    }

    done() {
        this.vehicles.forEach(vehicle => vehicle.done());
        this.vehicles.clear();
    }

    subscribe(event: MapVehiclesEvent, listener: MapVehiclesEventListener) {
        this.listeners.set(event, listener);
    }

    invoke(event: MapVehiclesEvent, vehicleId: string, position: Geodetic) {
        let cb = this.listeners.get(event);
        if (cb) cb(vehicleId, position);
    }

    setSelectedVehicle(vehicleId: string) {
        this.selectedVehicleId = vehicleId;
        this.vehicles.forEach((vehicle, id) => {
            vehicle.setSelected(id === vehicleId);
        });
    }

    addVehicle(vehicleId: string) {
        let vehicle = new MapVehicleCesium(vehicleId, this);
        this.vehicles.set(vehicleId, vehicle);
        vehicle.setSelected(vehicleId === this.selectedVehicleId)
        return vehicle;
    }

    removeVehicle(vehicleId: string) {
        this.vehicles.get(vehicleId)?.done();
        this.vehicles.delete(vehicleId);
    }

    vehicle(vehicleId: string) {
        return this.vehicles.get(vehicleId);
    }

    allVehicles() {
        return Array.from(this.vehicles.values());
    }

    vehicleIds() {
        return Array.from(this.vehicles.keys());
    }

    cesium: Cesium.Viewer;
    interaction: MapInteractionCesium;

    private selectedVehicleId: string;
    private vehicles: Map<string, MapVehicleCesium>
    private listeners: Map<MapVehiclesEvent, MapVehiclesEventListener>
}