import type { MapRuler } from '$lib/interfaces/map';
import type { MapInteractionCesium } from '$lib/map/cesium/interaction';
import type { Geodetic } from "$bindings/spatial"

import { GroundPointEntity, type EntityInputEvent } from "./base-entities"
import * as Utils from "./utils"

import * as Cesium from 'cesium'

class RulerPoint {
    constructor(ruler: MapRulerCesium, cesium: Cesium.Viewer, cartesian: Cesium.Cartesian3, interaction: MapInteractionCesium) {
        this.ruler = ruler;

        this.entity = new GroundPointEntity(cesium, 8);
        this.entity.baseColor = Cesium.Color.TURQUOISE;
        this.entity.setCartesian(cartesian);

        this.entity.subscribe((event: EntityInputEvent) => {
            if (event.DraggedPosition) {
                this.entity.setCartesian(event.DraggedPosition.cartesian);
            }
            if (event.Clicked) {
                this.ruler.removePoint(this);
            }
        });

        interaction.addInteractable(this.entity);
        interaction.hoverInteractable(this.entity);
    }

    done() {
        this.entity.done();
    }

    cartesian(): Cesium.Cartesian3 {
        return this.entity.cartesian;
    }

    setEnabled(enabled: boolean) {
        this.entity.draggable = enabled;
    }

    private entity: GroundPointEntity
    private ruler: MapRulerCesium
}

export class MapRulerCesium implements MapRuler {
    constructor(cesium: Cesium.Viewer, interaction: MapInteractionCesium) {
        this.cesium = cesium;
        this.interaction = interaction;

        this.points = [];
        this.labeledLines = [];
        this.enabled = false;

        interaction.subscribeClick((geodetic: Geodetic) => {
            if (this.enabled && geodetic) {
                const cartographic = Cesium.Cartographic.fromDegrees(geodetic.longitude, geodetic.latitude, geodetic.altitude);
                this.addPoint(Cesium.Cartographic.toCartesian(cartographic));
                return true;
            }
            return false;
        })
    }

    setEnabled(enabled: boolean): void {
        this.enabled = enabled;

        if (this.points.length === 1)
            this.clear();
        else
            this.points.forEach(point => { point.setEnabled(enabled); })
    }

    isEnabled(): boolean {
        return this.enabled;
    }

    clear(): void {
        for (let i = 0; i < this.labeledLines.length; ++i) {
            this.cesium.entities.remove(this.labeledLines[i]);
        }
        this.labeledLines = [];

        this.points.forEach(point => point.done());
        this.points = [];
    }

    distance(): number {
        let distance = 0;
        for (let i = 0; i < this.points.length - 1; i++) {
            distance += Cesium.Cartesian3.distance(this.points[i].cartesian(), this.points[i + 1].cartesian());
        }
        return distance;
    }

    addPoint(cartesian: Cesium.Cartesian3) {
        const previousPoint = this.points.slice(-1).pop();
        const newPoint = new RulerPoint(this, this.cesium, cartesian, this.interaction);
        newPoint.setEnabled(this.enabled);
        this.points.push(newPoint);

        if (previousPoint)
            this.addLabel(previousPoint, newPoint);
    }

    removePoint(point: RulerPoint) {
        const index = this.points.indexOf(point);
        if (index < 0)
            return;

        const hasRightBuddy = index + 1 < this.points.length;
        const hasLeftBuddy = index > 0;

        if (hasRightBuddy)
            this.removeLabel(index);
        if (hasLeftBuddy)
            this.removeLabel(index - 1);

        this.points[index].done();
        this.points.splice(index, 1);

        if (hasRightBuddy && hasLeftBuddy)
            this.addLabel(this.points[index - 1], this.points[index], index - 1);
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
            this.labeledLines.push(labeledLine);
        else
            this.labeledLines.splice(index, 0, labeledLine);
    }

    removeLabel(index: number) {
        this.cesium.entities.remove(this.labeledLines[index]);
        this.labeledLines.splice(index, 1);
    }

    private cesium: Cesium.Viewer
    private interaction: MapInteractionCesium

    private points: Array<RulerPoint>
    private labeledLines: Array<Cesium.Entity>
    private enabled: boolean
}
