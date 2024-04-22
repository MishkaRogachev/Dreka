import * as Cesium from "cesium";

import { MapInteractionCesium } from "$lib/map/cesium/interaction";
import { PylonEntity, BillboardEntity } from "$lib/map/cesium/base-entities"

import markerIcon from "$assets/svg/map_marker.svg";

export class MapMarker {
    constructor(cesium: Cesium.Viewer) {
        this.billboard = new BillboardEntity(cesium);
        this.billboard.icon = markerIcon;

        this.pylon = new PylonEntity(cesium, 4.0);
        this.pylon.baseColor = Cesium.Color.LIGHTSTEELBLUE;
    }

    done() {
        this.billboard.done();
        this.pylon.done();
    }


    setEnabled(enabled: boolean) {
        this.billboard.draggable = enabled;
    }

    setVisible(visible: boolean) {
        this.billboard.visible = visible;
        this.pylon.visible = visible;
    }

    setCartesian(cartesian: Cesium.Cartesian3) {
        this.billboard.setCartesian(cartesian);
        this.pylon.setCartesian(cartesian);
    }

    setBillboardColor(color: Cesium.Color) {
        this.billboard.baseColor = color;
    }

    setIcon(icon: string) {
        this.billboard.icon = icon;
    }

    isPositionValid(): boolean {
        return !this.billboard.cartesian.equals(Cesium.Cartesian3.ZERO)
    }

    cartesian(): Cesium.Cartesian3 {
        return this.billboard.cartesian
    }

    protected billboard: BillboardEntity
    protected pylon: PylonEntity
}

export class MapSign extends MapMarker {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        super(cesium);
        this.interaction = interaction;

        this.ordered = new MapMarker(cesium);
        this.ordered.setVisible(false);
    }

    done() {
        this.interaction.removeInteractable(this.billboard);
        this.ordered.done();
        super.done();
    }

    setDragCallback(cb: (cartesian: Cesium.Cartesian3) => void) {
        this.interaction.addInteractable(this.billboard);
        this.billboard.draggable = true;
        this.billboard.subscribeDragging((cartesian: Cesium.Cartesian3) => {
            this.billboard.opacity = 0.5;
            this.billboard.setHovered(false);
            this.ordered.setVisible(true);
            this.ordered.setBillboardColor(this.billboard.baseColor);
            this.ordered.setCartesian(cartesian);
        });
        this.billboard.subscribeDragged(() => {
            this.billboard.opacity = 1.0;
            this.ordered.setVisible(false);
            //this.ordered.setBillboardColor(Cesium.Color.GOLD);
            if (cb) cb(this.ordered.cartesian());
        });
    }

    setIcon(icon: string) {
        super.setIcon(icon);
        this.ordered.setIcon(icon);
    }

    // setOrderedCartesian(cartesian: Cesium.Cartesian3) {
    //     this.ordered.setBillboardColor(Cesium.Color.GOLD);
    //     this.ordered.setCartesian(cartesian);
    //     this.ordered.setVisible(true);
    // }

    protected ordered: MapMarker
    protected interaction: MapInteractionCesium
}
