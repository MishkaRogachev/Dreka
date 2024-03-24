import type { Cartesian } from "$bindings/spatial";
import { type Interactable, KeyModifier } from './interaction';

import * as Cesium from "cesium";

import * as Utils from "./utils";

const _hoveredFactor: number = 1.3;
const _draggingFacror: number = 1.6;

export class BaseEntity {
    constructor(cesium: Cesium.Viewer) {
        this.cesium = cesium;

        this._baseColor = Cesium.Color.WHITE;
        this._visible = true;
    }

    done() {}
    baseColor(): Cesium.Color { return this._baseColor; }
    setBaseColor(baseColor: Cesium.Color) { this._baseColor = baseColor; }
    setVisible(visible: boolean) { this._visible = visible; }

    protected cesium: Cesium.Viewer
    protected _baseColor: Cesium.Color
    protected _visible: boolean
}

export class BasePointEntity extends BaseEntity implements Interactable {
    constructor(cesium: Cesium.Viewer) {
        super(cesium);

        this._draggable = false;
        this._dragging = false;
        this._dragged = false;
        this._hovered = false;

        this._cartesian = Cesium.Cartesian3.ZERO;
        this._draggingListeners = [];
        this._draggedListeners = [];
        this._clickListeners = [];

        this._entity = this.cesium.entities.add({
            // @ts-ignore
            position: new Cesium.CallbackProperty(() => { return this._cartesian }, false),
        });
    }

    done() { this.cesium.entities.remove(this._entity); }

    addLabel(text: string) {
        this._entity.label = new Cesium.LabelGraphics({
            text: text,
            showBackground: true,
            backgroundColor: Cesium.Color.BLACK.withAlpha(0.9),
            pixelOffset: new Cesium.Cartesian2(0, -25),
            font: "12px sans-serif",
            disableDepthTestDistance: Number.POSITIVE_INFINITY
        });
    }
    removeLabel() { this._entity.label = undefined; }

    subscribeDragging(listener: Function) { this._draggingListeners.push(listener); }
    subscribeDragged(listener: Function) { this._draggedListeners.push(listener); }
    subscribeClick(listener: Function) { this._clickListeners.push(listener); }
    unsubscribeDragging(listener: Function) { this._draggingListeners = this._draggingListeners.filter(item => item !== listener); }
    unsubscribeDragged(listener: Function) { this._draggedListeners = this._draggedListeners.filter(item => item !== listener); }
    unsubscribeClick(listener: Function) { this._clickListeners = this._clickListeners.filter(item => item !== listener); }

    drag(screenXY: Cartesian, modifier: KeyModifier): boolean { return false; }
    click(): boolean {
        this._clickListeners.forEach(listener => listener(this._cartesian));
        return true;
    }

    setCartesian(cartesian: Cesium.Cartesian3) { this._cartesian = cartesian; }

    setDraggable(draggable: boolean) { this._draggable = draggable; }
    setHovered(hovered: boolean) { this._hovered = hovered; }
    setDragging(dragging: boolean) {
        if (this._dragging === dragging)
            return;

        this._dragging = dragging;
        if (!this._dragging && this._dragged) {
            this._draggedListeners.forEach(listener => listener(this._cartesian));
        }
    }

    matchInteraction(objects: Array<any>): boolean {
        return objects.find((object: any) => { return object.id === this._entity });
    }

    cartesian(): Cesium.Cartesian3 { return this._cartesian; }
    hasPosition(): boolean { return !this._cartesian.equals(Cesium.Cartesian3.ZERO); }
    isDraggable(): boolean { return this._draggable; }
    isDragging(): boolean { return this._dragging; }
    entity(): Cesium.Entity { return this._entity; }

    protected _entity: Cesium.Entity
    protected _cartesian: Cesium.Cartesian3

    protected _dragging: boolean
    protected _draggable: boolean
    protected _dragged: boolean
    protected _hovered: boolean

    protected _draggingListeners: Array<Function>
    protected _draggedListeners: Array<Function>
    protected _clickListeners: Array<Function>
}

export class GroundPointEntity extends BasePointEntity {
    constructor(cesium: Cesium.Viewer, radius: number) {
        super(cesium);

        this._entity.point = new Cesium.PointGraphics({
            pixelSize: new Cesium.CallbackProperty(() => {
                return this._dragging ? _draggingFacror * radius : this._hovered ? _hoveredFactor * radius : radius;
            }, false),
            color: new Cesium.CallbackProperty(() => { return this.baseColor() }, false)
        })
    }

    drag(screenXY: Cesium.Cartesian2, modifier: KeyModifier): boolean {
        if (modifier != KeyModifier.None)
            return false;

        const ray = this.cesium.camera.getPickRay(screenXY);
        if (!ray)
            return false;

        this._cartesian = this.cesium.scene.globe.pick(ray, this.cesium.scene) || this._cartesian;

        this._draggingListeners.forEach(listener => listener(this._cartesian));
        this._dragged = true;
        return true;
    }
}

export class BillboardEntity extends BasePointEntity {
    constructor(cesium: Cesium.Viewer) {
        super(cesium);

        this._icon = "";

        this._entity.billboard = new Cesium.BillboardGraphics({
            image: new Cesium.CallbackProperty(() => { return this._icon }, false),
            scale: new Cesium.CallbackProperty(() => {
                return this._dragging ? _draggingFacror : this._hovered ? _hoveredFactor : 1
            }, false),
            color: new Cesium.CallbackProperty(() => { return this.baseColor() }, false),
            disableDepthTestDistance: Number.POSITIVE_INFINITY,
            show: new Cesium.CallbackProperty(() => { return this.hasPosition() }, false),
        });
    }

    drag(screenXY: Cesium.Cartesian2, modifier: KeyModifier): boolean {
        if (modifier != KeyModifier.None && modifier != KeyModifier.Shift)
            return false;

        const scene = this.cesium.scene;

        // Normal by camera if any modifier, else normal by surface
        let normal = modifier == KeyModifier.Shift ?
            Cesium.Cartesian3.subtract(scene.camera.position, this._cartesian, new Cesium.Cartesian3()) :
            scene.globe.ellipsoid.geodeticSurfaceNormal(this._cartesian);

        if (!Cesium.defined(normal))
            return false;

        normal = Cesium.Cartesian3.normalize(normal, normal)

        // Cast ray from camera to plane projected by cartesian and normal
        const ray = scene.camera.getPickRay(screenXY);
        const plane = Cesium.Plane.fromPointNormal(this._cartesian, normal);
        const cartesian = Cesium.IntersectionTests.rayPlane(ray!, plane);
        let newGeodetic = scene.globe.ellipsoid.cartesianToCartographic(cartesian);

        if (modifier == KeyModifier.Shift) {
            const geodetic = scene.globe.ellipsoid.cartesianToCartographic(this._cartesian);
            newGeodetic.latitude = geodetic.latitude;
            newGeodetic.longitude = geodetic.longitude;
        }

        this.setCartesian(scene.globe.ellipsoid.cartographicToCartesian(newGeodetic));
        this._draggingListeners.forEach(listener => listener(this._cartesian));
        this._dragged = true;

        return true;
    }

    setIcon(icon: string) { this._icon = icon ;}

    private _icon: string
}

export class PylonEntity extends BasePointEntity {
    constructor(cesium: Cesium.Viewer, width: number) {
        super(cesium);

        this._terrainCartesian = Cesium.Cartesian3.ZERO;
        this._terrainListeners = [];

        this._entity.polyline = new Cesium.PolylineGraphics({
            positions: new Cesium.CallbackProperty(() => {
                return [this._cartesian, this._terrainCartesian]
            }, false),
            arcType: Cesium.ArcType.NONE,
            material: new Cesium.PolylineArrowMaterialProperty(
                new Cesium.CallbackProperty(() => { return this.baseColor() }, false)
            ),
            width: width
        });
    }

    subscribeTerrain(listener: Function) { this._terrainListeners.push(listener); }
    unsubscribeTerrain(listener: Function) { this._terrainListeners = this._terrainListeners.filter(item => item !== listener); }

    setCartesian(cartesian: Cesium.Cartesian3) {
        super.setCartesian(cartesian);

        // Sample terrain position from the ground
        if (this.hasPosition()) {
            const cartographic = Cesium.Cartographic.fromCartesian(this._cartesian);
            const promise = Cesium.sampleTerrainMostDetailed(this.cesium.terrainProvider, [cartographic]);
            promise.then(updatedPositions => {
                this._terrainCartesian = Cesium.Cartographic.toCartesian(cartographic)
                const terrainAltitude = cartographic.height
                this._terrainListeners.forEach(listener => listener(terrainAltitude))
            });
        }
    }

    private _terrainCartesian: Cesium.Cartesian3
    protected _terrainListeners: Array<Function>
}

export class CircleEntity extends BasePointEntity {
    constructor(cesium: Cesium.Viewer, width: number) {
        super(cesium);

        this._width = width;
        this._radius = undefined;
        this._height = 0.0;

        this._entity.ellipse = new Cesium.EllipseGraphics({
            fill: false,
            height: new Cesium.CallbackProperty(() => { return this._height }, false),
            semiMinorAxis: new Cesium.CallbackProperty(() => { return this._radius }, false),
            semiMajorAxis: new Cesium.CallbackProperty(() => { return this._radius }, false),
            outline: true,
            outlineWidth: new Cesium.CallbackProperty(() => {
                return this._hovered ? this._width * _hoveredFactor : this._width
            }, false),
            outlineColor: new Cesium.CallbackProperty(() => { return this.baseColor() }, false)
        });
    }

    drag(screenXY: Cesium.Cartesian2, modifier: KeyModifier): boolean {
        if (modifier != KeyModifier.None)
            return false;

        const scene = this.cesium.scene;

        // Normal by camera if any modifier, else normal by surface
        let normal = scene.globe.ellipsoid.geodeticSurfaceNormal(this._cartesian);
        if (!Cesium.defined(normal))
            return false;

        normal = Cesium.Cartesian3.normalize(normal, normal);

        // Cast ray from camera to plane projected by cartesian and normal
        const ray = scene.camera.getPickRay(screenXY);
        const plane = Cesium.Plane.fromPointNormal(this._cartesian, normal);
        const cartesian = Cesium.IntersectionTests.rayPlane(ray!, plane);

        this._radius = Cesium.Cartesian3.distance(cartesian, this._cartesian);

        this._draggingListeners.forEach(listener => listener(this._radius));
        this._dragged = true;
        return true;
    }

    setDragging(dragging: boolean) {
        if (this._dragging === dragging)
            return;

        this._dragging = dragging;
        if (!dragging) {
            this._draggedListeners.forEach(listener => listener(this._radius));
        }
    }

    setWidth(width: number) { this._width = width; }
    setRadius(radius: number) { this._radius = radius; }
    setHeight(height: number) { this._height = height; }

    private _width: number
    private _radius: number | undefined
    private _height: number
}

export class ModelEntity extends BasePointEntity {
    constructor(cesium: Cesium.Viewer) {
        super(cesium);

        this._model = "";
        this._silhouetteColor = Cesium.Color.GRAY;

        this._hpr = new Cesium.HeadingPitchRoll(0, 0, 0);

        this._entity.orientation = new Cesium.CallbackProperty(() => {
            return Cesium.Transforms.headingPitchRollQuaternion(this._cartesian, this._hpr);
        }, false);

        this._entity.model = new Cesium.ModelGraphics({
            uri: new Cesium.CallbackProperty(() => { return this._model }, false),
            color: new Cesium.CallbackProperty(() => { return this.baseColor() }, false),
            silhouetteColor: new Cesium.CallbackProperty(() => { return this._silhouetteColor }, false),
            minimumPixelSize: 196,
            maximumScale: 80000,
            colorBlendMode: Cesium.ColorBlendMode.REPLACE,
            silhouetteSize: 2.0
        });
    }

    setUri(model: string) { this._model = model }
    setSilhouetteColor(silhouetteColor: Cesium.Color) { this._silhouetteColor = silhouetteColor; }

    setHpr(heading: number, pitch: number, roll: number) {
        this._hpr = new Cesium.HeadingPitchRoll(Cesium.Math.toRadians(heading - 90),
                                                Cesium.Math.toRadians(pitch),
                                                Cesium.Math.toRadians(roll));
    }

    private _hpr: Cesium.HeadingPitchRoll;
    private _model: string;
    private _silhouetteColor: Cesium.Color;
}

export class PathEntity extends BaseEntity {
    constructor(cesium: Cesium.Viewer, length: number) {
        super(cesium);

        this._length = length;
        this._track = [];
    }

    done() { this.clear(); }

    clear() {
        this._track.forEach(entity => this.cesium.entities.remove(entity));
        this._track = [];
    }

    setVisible(visible: boolean) {
        this._track.forEach(entity => entity.show = visible);
    }

    addCartesian(cartesian: Cesium.Cartesian3) {
        const point = this.cesium.entities.add({
            position: cartesian,
            point: {
                pixelSize : 4,
                color: new Cesium.CallbackProperty(() => { return this.baseColor() }, false),
            }
        });
        this._track.push(point);

        // Remove extra points
        if (this._length >= 0) {
            let pointsToClear = this._track.length - this._length;
            if (pointsToClear > 0) {
                for (var i = 0; i < pointsToClear; ++i) {
                    this.cesium.entities.remove(this._track.shift()!);
                }
            }
        }
    }

    setCartesians(cartesians: Array<Cesium.Cartesian3>) {
        this.clear();
        cartesians.forEach(cartesian=> { this.addCartesian(cartesian) });
    }

    private _length: number
    private _track: Array<Cesium.Entity>
}

export class ProjectionEntity extends BaseEntity {
    constructor(cesium: Cesium.Viewer) {
        super(cesium);

        this._sensorRoot = Cesium.Cartesian3.ZERO;
        this._left = Cesium.Cartesian3.ZERO;
        this._right = Cesium.Cartesian3.ZERO;

        this._distance = 10000;
        this._azimuth = 0;

        this._direction = this.cesium.entities.add({
            polygon: {
                hierarchy: new Cesium.CallbackProperty(() => {
                    let array = []
                    if (this._left !== Cesium.Cartesian3.ZERO)
                        array.push(this._left)
                    if (this._right !== Cesium.Cartesian3.ZERO)
                        array.push(this._right)
                    if (this._sensorRoot !== Cesium.Cartesian3.ZERO)
                        array.push(this._sensorRoot)
                    return new Cesium.PolygonHierarchy(array)
                }, false),
                material: new Cesium.StripeMaterialProperty({
                    evenColor: new Cesium.CallbackProperty(() => { return this.baseColor().withAlpha(0.25) }, false),
                    oddColor: Cesium.Color.TRANSPARENT,
                }),
                stRotation: new Cesium.CallbackProperty(() => { return Cesium.Math.toRadians(this._azimuth) }, false),
                perPositionHeight: true,
                arcType: Cesium.ArcType.GEODESIC
            }
        });
    }

    done() { this.cesium.entities.remove(this._direction); }

    setCartesian(cartesian: Cesium.Cartesian3) { this._sensorRoot = cartesian; }
    setDirection(azimuth: number, elevation: number, fov: number) {
        this._azimuth = azimuth;

        this._left = this._castRayDistanceLimitedBySurface(azimuth - fov / 2, elevation);
        this._right = this._castRayDistanceLimitedBySurface(azimuth + fov / 2, elevation);
    }
    serDistance(distance: number) { this._distance = distance; }

    _castRayDistance(azimuth: number, elevation: number): Cesium.Cartesian3 {
        const hpr = Utils.hprFromAttitude(azimuth, elevation);
        const ray = Utils.castRay(this._sensorRoot, hpr);
        return Cesium.Ray.getPoint(ray, this._distance);
    }

    _castRayToSurface(azimuth: number, elevation: number): Cesium.Cartesian3 {
        const hpr = Utils.hprFromAttitude(azimuth, elevation);
        const ray = Utils.castRay(this._sensorRoot, hpr);
        return this.cesium.scene.globe.pick(ray, this.cesium.scene) || Cesium.Cartesian3.ZERO;
    }

    _castRayDistanceLimitedBySurface(azimuth: number, elevation: number): Cesium.Cartesian3 {
        const cartesian = this._castRayToSurface(azimuth, elevation);
        if (cartesian && cartesian !== Cesium.Cartesian3.ZERO)
            return cartesian;

        return this._castRayDistance(azimuth, elevation);
    }

    protected _sensorRoot: Cesium.Cartesian3
    private _left: Cesium.Cartesian3
    private _right: Cesium.Cartesian3

    private _distance: number
    private _azimuth: number

    private _direction: Cesium.Entity
}

export class RectProjectionEntity extends ProjectionEntity {
    constructor(cesium: Cesium.Viewer) {
        super(cesium);

        this._maxElevation = -15;

        this._topLeft = Cesium.Cartesian3.ZERO;
        this._topRight = Cesium.Cartesian3.ZERO;
        this._bottomRight = Cesium.Cartesian3.ZERO;
        this._bottomLeft = Cesium.Cartesian3.ZERO;

        this._projection = this.cesium.entities.add({
            polygon: {
                hierarchy: new Cesium.CallbackProperty(() => {
                    let array = [];
                    if (this._topLeft !== Cesium.Cartesian3.ZERO)
                        array.push(this._topLeft);
                    if (this._topRight !== Cesium.Cartesian3.ZERO)
                        array.push(this._topRight);
                    if (this._bottomRight !== Cesium.Cartesian3.ZERO)
                        array.push(this._bottomRight);
                    if (this._bottomLeft !== Cesium.Cartesian3.ZERO)
                        array.push(this._bottomLeft);
                    return new Cesium.PolygonHierarchy(array);
                }, false),
                outline: true,
                outlineWidth: 8.0,
                outlineColor: new Cesium.CallbackProperty(() => { return this._baseColor }, false),
                material: new Cesium.ColorMaterialProperty(new Cesium.CallbackProperty(() => {
                    return this.baseColor().withAlpha(0.25)
                }, false)),
                perPositionHeight: true,
                arcType: Cesium.ArcType.GEODESIC
            }
        });
    }

    done() {
        super.done();
        this.cesium.entities.remove(this._projection);
    }

    setProjection(azimuth: number, elevation: number, horizontal_fov: number, vertical_fov: number) {
        this.setDirection(azimuth, elevation, horizontal_fov);

        // Show only pointing lower
        if (elevation < this._maxElevation) {
            this._topLeft = this._castRayToSurface(azimuth - horizontal_fov / 2, elevation - vertical_fov / 2);
            this._topRight = this._castRayToSurface(azimuth + horizontal_fov / 2, elevation - vertical_fov / 2);
            this._bottomRight = this._castRayToSurface(azimuth + horizontal_fov / 2, elevation + vertical_fov / 2);
            this._bottomLeft = this._castRayToSurface(azimuth - horizontal_fov / 2, elevation + vertical_fov / 2);
        } else {
            this._topLeft = Cesium.Cartesian3.ZERO;
            this._topRight = Cesium.Cartesian3.ZERO;
            this._bottomRight = Cesium.Cartesian3.ZERO;
            this._bottomLeft = Cesium.Cartesian3.ZERO;
        }
    }

    private _maxElevation: number

    private _topLeft: Cesium.Cartesian3
    private _topRight: Cesium.Cartesian3
    private _bottomRight: Cesium.Cartesian3
    private _bottomLeft: Cesium.Cartesian3

    private _projection: Cesium.Entity
}
