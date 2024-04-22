import * as Cesium from "cesium";

import { MapInteractionCesium } from "$lib/map/cesium/interaction";
import { PylonEntity, BillboardEntity } from "$lib/map/cesium/base-entities"

import markerIcon from "$assets/svg/map_marker.svg";

export class MapMarker {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.interaction = interaction;
        this.cesium = cesium;

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

    isPositionValid(): boolean {
        return !this.billboard.cartesian.equals(Cesium.Cartesian3.ZERO)
    }

    cartesian(): Cesium.Cartesian3 {
        return this.billboard.cartesian
    }

    billboard: BillboardEntity
    pylon: PylonEntity

    protected interaction: MapInteractionCesium
    protected cesium: Cesium.Viewer
}

export class MapSign extends MapMarker {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        super(cesium, interaction);

        this.billboard.subscribeDragging((cartesian: Cesium.Cartesian3) => {
            this.billboard.opacity = 0.5;
            this.billboard.setHovered(false);
            this.ordered.setVisible(true);
            this.ordered.setBillboardColor(this.billboard.baseColor);
            this.ordered.setCartesian(cartesian);
        });

        this.ordered = new MapMarker(cesium, interaction);
        this.ordered.setVisible(false);
        this.ordered.billboard.subscribeDragging((cartesian: Cesium.Cartesian3) => {
            this.ordered.setCartesian(cartesian);
        });

        this.line = cesium.entities.add({
            polyline: {
                positions: new Cesium.CallbackProperty(() => {
                    if (this.isPositionValid() && this.ordered.isPositionValid() && this.ordered.billboard.visible)
                        return [this.cartesian(), this.ordered.cartesian()]
                    return []
                }, false),
                arcType: Cesium.ArcType.GEODESIC,
                material: new Cesium.PolylineArrowMaterialProperty(
                    new Cesium.CallbackProperty(() => { return this.billboard.baseColor; }, false),
                ),
                width: 4.0
            }
        })
    }

    done() {
        this.cesium.entities.remove(this.line);
        this.ordered.done();
        super.done();
    }

    setDragCallback(cb: (cartesian: Cesium.Cartesian3) => void) {
        this.billboard.subscribeDragged(() => {
            if (cb) cb(this.ordered.cartesian());
        });
        this.ordered.billboard.subscribeDragged(() => {
            if (cb) cb(this.ordered.cartesian());
        });
    }

    setIcon(icon: string) {
        super.setIcon(icon);
        this.ordered.setIcon(icon);
    }

    setCartesian(cartesian: Cesium.Cartesian3) {
        super.setCartesian(cartesian);

        if (Cesium.Cartesian3.equalsEpsilon(this.ordered.cartesian(), cartesian, Cesium.Math.EPSILON6)) {
            this.billboard.opacity = 1.0;
            this.ordered.setVisible(false);
            this.ordered.setCartesian(Cesium.Cartesian3.ZERO);
        }
    }

    setOrdredColor(color: Cesium.Color) {
        this.ordered.setBillboardColor(color);
    }

    protected ordered: MapMarker
    protected line: Cesium.Entity
}
