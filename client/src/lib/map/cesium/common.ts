import * as Cesium from "cesium";

import { MapInteractionCesium } from "$lib/map/cesium/interaction";
import { PylonEntity, BillboardEntity, type EntityInputEvent } from "$lib/map/cesium/base-entities"

import markerIcon from "$assets/svg/map_marker.svg";

export class MapMarker {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.interaction = interaction;

        this.billboard = new BillboardEntity(cesium);
        this.billboard.icon = markerIcon;
        this.billboard.draggable = true;
        this.interaction.addInteractable(this.billboard);

        this.pylon = new PylonEntity(cesium, 4.0);
        this.pylon.baseColor = Cesium.Color.LIGHTSTEELBLUE;
    }

    done() {
        this.interaction.removeInteractable(this.billboard);
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

    isVisibleAndPositionValid(): boolean {
        return this.billboard.visible && !this.billboard.cartesian.equals(Cesium.Cartesian3.ZERO)
    }

    cartesian(): Cesium.Cartesian3 {
        return this.billboard.cartesian
    }

    billboard: BillboardEntity
    pylon: PylonEntity

    protected interaction: MapInteractionCesium
}

export class MapSign extends MapMarker {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        super(cesium, interaction);

        this.billboard.subscribe((event: EntityInputEvent) => {
            if (event.DragStarted) {
                this.billboard.opacity = 0.5;
                this.pylon.opacity = 0.5;
                this.billboard.setHovered(false);
                this.ordered.setVisible(true);
                this.ordered.setBillboardColor(this.billboard.baseColor);
            }
            if (event.DraggedPosition) {
                this.ordered.setCartesian(event.DraggedPosition.cartesian);
            }
            if (event.DragCompleted) {
                this.billboard.opacity = 1.0;
                this.pylon.opacity = 1.0;
                if (this.dragCallBack) this.dragCallBack(this.ordered.cartesian());
            }
        });

        this.ordered = new MapMarker(cesium, interaction);
        this.ordered.setVisible(false);
        this.ordered.billboard.subscribe((event: EntityInputEvent) => {
            if (event.DraggedPosition) {
                this.ordered.setCartesian(event.DraggedPosition.cartesian);
            }
            if (event.DragCompleted) {
                if (this.dragCallBack) this.dragCallBack(this.ordered.cartesian());
            }
        });
    }

    done() {
        this.ordered.done();
        super.done();
    }

    setDragCallback(dragCallBack: (cartesian: Cesium.Cartesian3) => void) {
        this.dragCallBack = dragCallBack;
    }

    setIcon(icon: string) {
        super.setIcon(icon);
        this.ordered.setIcon(icon);
    }

    setCartesian(cartesian: Cesium.Cartesian3) {
        super.setCartesian(cartesian);

        if (Cesium.Cartesian3.equalsEpsilon(this.ordered.cartesian(), cartesian, Cesium.Math.EPSILON6)) {
            this.billboard.opacity = 1.0;
            this.pylon.opacity = 1.0;
            this.ordered.setVisible(false);
            this.ordered.setCartesian(Cesium.Cartesian3.ZERO);
        }
    }

    setOrdredColor(color: Cesium.Color) {
        this.ordered.setBillboardColor(color);
    }

    ordered: MapMarker

    private dragCallBack?: (cartesian: Cesium.Cartesian3) => void
}
