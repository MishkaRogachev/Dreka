import type { MapRuler } from '$lib/interfaces/map';
import type { MapInteractionCesium } from '$lib/map/cesium/interaction';
import type { Geodetic } from "$lib/interfaces/common"

import { GroundPointEntity } from "./base-entities"
import * as Utils from "./utils"

import * as Cesium from 'cesium'

class RulerPoint {
    constructor(ruler: MapRulerCesium, cesium: Cesium.Viewer, cartesian: Cesium.Cartesian3, interaction: MapInteractionCesium) {
        this.ruler = ruler;

        this.entity = new GroundPointEntity(cesium, 8);
        this.entity.setCartesian(cartesian);
        this.entity.setDraggable(true);
        this.entity.setBaseColor(Cesium.Color.TURQUOISE);

        interaction.addInteractable(this.entity);
        interaction.hoverInteractable(this.entity);

        this.entity.subscribeClick(() => {
            this.ruler.removePoint(this);
        })
    }

    done() {
        this.entity.done();
    }

    cartesian(): Cesium.Cartesian3 {
        return this.entity.cartesian();
    }

    setEnabled(enabled: boolean) {
        this.entity.setDraggable(enabled);
    }

    private entity: GroundPointEntity
    private ruler: MapRulerCesium
}

export class MapRulerCesium implements MapRuler {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.cesium = cesium;
        this._interaction = interaction;

        this._points = [];
        this._labeledLines = [];
        this._enabled = false;

        interaction.subscribeClick((geodetic: Geodetic) => {
            if (this._enabled && geodetic) {
                const cartographic = Cesium.Cartographic.fromDegrees(geodetic.longitude, geodetic.latitude, geodetic.altitude);
                this.addPoint(Cesium.Cartographic.toCartesian(cartographic));
                return true;
            }
            return false;
        })
    }

    setEnabled(enabled: boolean): void {
        this._enabled = enabled;

        if (this._points.length === 1)
            this.clear();
        else
            this._points.forEach(point => { point.setEnabled(enabled); })
    }

    enabled(): boolean {
        return this._enabled;
    }

    clear(): void {
        for (let i = 0; i < this._labeledLines.length; ++i) {
            this.cesium.entities.remove(this._labeledLines[i]);
        }
        this._labeledLines = [];

        this._points.forEach(point => point.done());
        this._points = [];
    }

    distance(): number {
        let distance = 0;
        for (let i = 0; i < this._points.length - 1; i++) {
            distance += Cesium.Cartesian3.distance(this._points[i].cartesian(), this._points[i + 1].cartesian());
        }
        return distance;
    }

    addPoint(cartesian: Cesium.Cartesian3) {
        const previousPoint = this._points.slice(-1).pop();
        const newPoint = new RulerPoint(this, this.cesium, cartesian, this._interaction);
        this._points.push(newPoint);

        if (previousPoint)
            this.addLabel(previousPoint, newPoint);
    }

    removePoint(point: RulerPoint) {
        const index = this._points.indexOf(point);
        if (index < 0)
            return;

        const hasRightBuddy = index + 1 < this._points.length;
        const hasLeftBuddy = index > 0;

        if (hasRightBuddy)
            this.removeLabel(index);
        if (hasLeftBuddy)
            this.removeLabel(index - 1);

        this._points[index].done();
        this._points.splice(index, 1);

        if (hasRightBuddy && hasLeftBuddy)
            this.addLabel(this._points[index - 1], this._points[index], index - 1);
    }

    addLabel(first: RulerPoint, second: RulerPoint, index = -1) {
        const labeledLine = this.cesium.entities.add({
            //@ts-ignore
            position: new Cesium.CallbackProperty(() => { return Utils.intermediate(first.cartesian(), second.cartesian()) }, false),
            polyline: {
                positions: new Cesium.CallbackProperty(() => { return [first.cartesian(), second.cartesian()] }, false),
                arcType: Cesium.ArcType.GEODESIC,
                width: 3,
                material: Cesium.Color.TURQUOISE
            },
            label: {
                text: new Cesium.CallbackProperty(() => { return Math.round(Cesium.Cartesian3.distance(
                                                                    first.cartesian(), second.cartesian())) + " m" }, false),
                showBackground: true,
                disableDepthTestDistance: Number.POSITIVE_INFINITY,
                backgroundColor: Cesium.Color.TURQUOISE,
                fillColor: Cesium.Color.BLACK,
                font: "12px sans-serif"
            }
        });

        if (index === -1)
            this._labeledLines.push(labeledLine);
        else
            this._labeledLines.splice(index, 0, labeledLine);
    }

    removeLabel(index: number) {
        this.cesium.entities.remove(this._labeledLines[index]);
        this._labeledLines.splice(index, 1);
    }

    private cesium: Cesium.Viewer
    private _interaction: MapInteractionCesium

    private _points: Array<RulerPoint>
    private _labeledLines: Array<Cesium.Entity>
    private _enabled: boolean
}
