import { GeodeticFrame, type Geodetic } from '$bindings/spatial';
import { VehicleMode, type VehicleDescription, type VehicleStatus } from '$bindings/vehicles';
import type { Flight, Navigation } from '$bindings/telemetry';
import { toColorCode } from '$bindings/colors';

import { cartesianFromGeodetic, geodeticFromCartesian } from '$lib/map/cesium/utils';
import { type MapVehiclesEvent, type MapVehicle, type MapVehicles, type MapVehiclesEventListener } from '$lib/interfaces/map';
import { MapInteractionCesium } from '$lib/map/cesium/interaction';
import { ModelEntity, PylonEntity, PathEntity, type EntityInputEvent } from "$lib/map/cesium/base-entities"
import { MapSign } from '$lib/map/cesium/common';

import * as Cesium from 'cesium';

import homeIcon from "$assets/svg/map_home.svg";
import targetIcon from "$assets/svg/map_target_wpt.svg";
import wptIcon from "$assets/svg/map_wpt.svg";

// @ts-ignore
import fixedWing from "$assets/3d/art_v1.glb";

export class MapVehicleCesium implements MapVehicle {
    constructor(vehicleId: string, parent: MapVehiclesCesium) {
        this.vehicleId = vehicleId;
        this.parent = parent;

        this.model = new ModelEntity(parent.cesium);
        this.model.hoverable = true;
        parent.interaction.addInteractable(this.model);
        this.model.subscribe((event: EntityInputEvent) => {
            if (event.Hovered) {
                this.parent.invoke({ VehicleHovered: { vehicleId: this.vehicleId, hovered: true } });
            } else if (event.Exited) {
                this.parent.invoke({ VehicleHovered: { vehicleId: this.vehicleId, hovered: false } });
            }
            else if (event.Clicked) {
                this.parent.invoke({ ActivateVehicle: { vehicleId: this.vehicleId } });
            }
        });

        this.pylon = new PylonEntity(parent.cesium);
        this.path = new PathEntity(parent.cesium, 100);

        this.target = new MapSign(parent.cesium, parent.interaction);
        this.target.setBillboardColor(Cesium.Color.MAGENTA);
        this.target.setDragCallback((cartesian: Cesium.Cartesian3) => {
            const geodetic = geodeticFromCartesian(cartesian, GeodeticFrame.Wgs84AboveSeaLevel, 0);
            if (geodetic) {
                this.target.setOrdredColor(Cesium.Color.GOLD); // TODO: indicate ack with color
                this.parent.invoke({ TargetPositionOrdered: { vehicleId: this.vehicleId, position: geodetic } });
            }
        });
        this.target.billboard.subscribe((event: EntityInputEvent) => {
            if (event.Hovered) {
                this.parent.invoke({ TargetHovered: { vehicleId: this.vehicleId, hovered: true } });
            } else if (event.Exited) {
                this.parent.invoke({ TargetHovered: { vehicleId: this.vehicleId, hovered: false } });
            }
        });

        this.home = new MapSign(parent.cesium, parent.interaction);
        this.home.setIcon(homeIcon);
        this.home.setDragCallback((cartesian: Cesium.Cartesian3) => {
            const geodetic = geodeticFromCartesian(cartesian, GeodeticFrame.Wgs84AboveSeaLevel, 0);
            if (geodetic) {
                this.home.setOrdredColor(Cesium.Color.GOLD); // TODO: indicate ack with color
                this.parent.invoke({ HomePositionOrdered: { vehicleId: this.vehicleId, position: geodetic } });
            }
        });
        this.home.billboard.subscribe((event: EntityInputEvent) => {
            if (event.Hovered) {
                this.parent.invoke({ HomeHovered: { vehicleId: this.vehicleId, hovered: true } });
            } else if (event.Exited) {
                this.parent.invoke({ HomeHovered: { vehicleId: this.vehicleId, hovered: false } });
            }
        });
    }

    done() {
        this.home.done();
        this.target.done();

        this.path.done();
        this.pylon.done();

        this.parent.interaction.removeInteractable(this.model);
        this.model.done();
    }

    cartesian(): Cesium.Cartesian3 {
        return this.model.cartesian;
    }

    centerOnMap() {
        this.model.centerOnMap();
    }

    setTracking(tracking: boolean) {
        this.model.setTracking(tracking);
    }

    updateFromDescription(vehicle: VehicleDescription) {
        this.model.modelUri = fixedWing; // TODO: get from type

        const color = Cesium.Color.fromCssColorString(toColorCode(vehicle.color));
        this.model.baseColor = color;
        this.pylon.baseColor = color;
        this.path.baseColor = color;
    }

    updateFromStatus(status: VehicleStatus | undefined) {
        // TODO: online fading

        switch (status?.mode) {
            case VehicleMode.Guided:
                this.target.setIcon(targetIcon);
                this.target.setVisible(true);
                this.target.setEnabled(true);
                break;
            case VehicleMode.Loiter:
                this.target.setIcon(wptIcon);
                this.target.setVisible(true);
                this.target.setEnabled(false);
                break;
            default:
                this.target.setVisible(false);
                this.target.setEnabled(false);
                break;
        }
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

        if (navigation.target_position.latitude === navigation.home_position.latitude &&
            navigation.target_position.longitude === navigation.home_position.longitude) {
            this.home.setBillboardColor(Cesium.Color.MAGENTA)
        } else {
            const targetCartesian = cartesianFromGeodetic(navigation.target_position, 0);
            this.target.setCartesian(targetCartesian);
            this.home.setBillboardColor(Cesium.Color.WHITE)
        }
    }

    setSelected(selected: boolean) {
        this.model.silhouetteColor = selected ? Cesium.Color.WHITE : Cesium.Color.GRAY;
        this.path.visible = selected;
    }

    private vehicleId: string;
    private parent: MapVehiclesCesium;

    private path: PathEntity
    private model: ModelEntity
    private pylon: PylonEntity

    private home: MapSign
    private target: MapSign
}

export class MapVehiclesCesium implements MapVehicles {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.cesium = cesium;
        this.interaction = interaction;

        this.selectedVehicleId = "";
        this.vehicles = new Map();
        this.listeners = [];
    }

    done() {
        this.vehicles.forEach(vehicle => vehicle.done());
        this.vehicles.clear();
    }

    subscribe(listener: MapVehiclesEventListener) {
        this.listeners.push(listener);
    }

    unsubscribe(listener: MapVehiclesEventListener) {
        this.listeners = this.listeners.filter(l => l !== listener);
    }

    invoke(event: MapVehiclesEvent) {
        this.listeners.forEach(listener => listener(event));
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
    private listeners: Array<MapVehiclesEventListener>
}
