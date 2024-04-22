import * as Cesium from "cesium";

import type { Cartesian } from "$bindings/spatial";

import { type Interactable, KeyModifier } from '$lib/map/cesium/interaction';
import * as Utils from "$lib/map/cesium/utils";

const HOVER_SCALE_MULTIPLIER = 1.35;
const MAX_ELEVATION_DEFAULT = -15;

export class BaseEntity {
    constructor(cesium: Cesium.Viewer) {
        this.cesium = cesium;

        this.baseColor = Cesium.Color.WHITE;
        this.opacity = 1.0;
        this.visible = true;
    }

    done() {}

    baseColor: Cesium.Color
    opacity: number
    visible: boolean

    protected cesium: Cesium.Viewer
}

export class BasePointEntity extends BaseEntity implements Interactable {
    constructor(cesium: Cesium.Viewer) {
        super(cesium);

        this.draggable = false;
        this.dragging = false;
        this.hovered = false;

        this.cartesian = Cesium.Cartesian3.ZERO;
        this.draggingListeners = [];
        this.draggedListeners = [];
        this.clickListeners = [];

        this.entity = this.cesium.entities.add({
            // @ts-ignore
            position: new Cesium.CallbackProperty(() => { return this.cartesian }, false),
        });
    }

    done() { this.cesium.entities.remove(this.entity); }

    addLabel(text: string) {
        this.entity.label = new Cesium.LabelGraphics({
            text: text,
            showBackground: true,
            backgroundColor: Cesium.Color.fromCssColorString("#111827"),
            pixelOffset: new Cesium.Cartesian2(0, -25),
            font: "10px Helvetica",
            disableDepthTestDistance: Number.POSITIVE_INFINITY,
            show: new Cesium.CallbackProperty(() => { return this.hasPosition() && this.visible }, false),
        });
    }
    removeLabel() { this.entity.label = undefined; }

    centerOnMap() {
        const distance = Cesium.Cartesian3.distance(this.cartesian, this.cesium.camera.positionWC);
        this.cesium.zoomTo(this.entity, new Cesium.HeadingPitchRange(
            this.cesium.camera.heading, this.cesium.camera.pitch, distance));
    }

    setTracking(tracking: boolean) {
        BasePointEntity.trackingEnity = tracking ? this : undefined;
        this.centerOnMap();

        // TODO: enable zoom & tilt
        this.cesium.scene.screenSpaceCameraController.enableInputs = !tracking;

        // NOTE: this is fallback generic implementation
        // this.cesium.trackedEntity = tracking ? this.entity : undefined;
    }

    // TODO: enum for subscribitions
    subscribeDragging(listener: (cartesian: Cesium.Cartesian3) => void) { this.draggingListeners.push(listener); }
    subscribeDragged(listener: () => void) { this.draggedListeners.push(listener); }
    subscribeClick(listener: Function) { this.clickListeners.push(listener); }
    unsubscribeDragging(listener: Function) { this.draggingListeners = this.draggingListeners.filter(item => item !== listener); }
    unsubscribeDragged(listener: Function) { this.draggedListeners = this.draggedListeners.filter(item => item !== listener); }
    unsubscribeClick(listener: Function) { this.clickListeners = this.clickListeners.filter(item => item !== listener); }

    drag(screenXY: Cartesian, modifier: KeyModifier): boolean { return false; }
    click(): boolean {
        this.clickListeners.forEach(listener => listener(this.cartesian));
        return true;
    }

    setCartesian(cartesian: Cesium.Cartesian3) {
        this.cartesian = cartesian;
        if (BasePointEntity.trackingEnity == this) {
            this.centerOnMap();
        }
    }

    setHovered(hovered: boolean) { this.hovered = hovered; }
    setDragging(dragging: boolean) {
        if (this.dragging === dragging)
            return;

        this.dragging = dragging;
        if (!this.dragging) {
            this.draggedListeners.forEach(listener => listener());
        }
    }

    matchInteraction(objects: Array<any>): boolean {
        return objects.find((object: any) => { return object.id === this.entity });
    }

    hasPosition(): boolean { return !this.cartesian.equals(Cesium.Cartesian3.ZERO); }
    isDraggable(): boolean { return this.draggable; }
    isDragging(): boolean { return this.dragging; }

    protected entity: Cesium.Entity
    cartesian: Cesium.Cartesian3
    draggable: boolean

    protected dragging: boolean
    protected hovered: boolean

    protected draggingListeners: Array<Function>
    protected draggedListeners: Array<Function>
    protected clickListeners: Array<Function>

    private static trackingEnity: BasePointEntity | undefined
}

export class GroundPointEntity extends BasePointEntity {
    constructor(cesium: Cesium.Viewer, radius: number) {
        super(cesium);

        this.entity.point = new Cesium.PointGraphics({
            pixelSize: new Cesium.CallbackProperty(() => {
                return this.dragging || this.hovered ? HOVER_SCALE_MULTIPLIER * radius : radius;
            }, false),
            color: new Cesium.CallbackProperty(() => { return this.baseColor.withAlpha(this.opacity) }, false)
        })
    }

    drag(screenXY: Cesium.Cartesian2, modifier: KeyModifier): boolean {
        if (modifier != KeyModifier.None)
            return false;

        const ray = this.cesium.camera.getPickRay(screenXY);
        if (!ray)
            return false;

        const cartesian = this.cesium.scene.globe.pick(ray, this.cesium.scene) || this.cartesian;
        this.draggingListeners.forEach(listener => listener(cartesian));
        return true;
    }
}

export class BillboardEntity extends BasePointEntity {
    constructor(cesium: Cesium.Viewer) {
        super(cesium);

        this.icon = "";

        this.entity.billboard = new Cesium.BillboardGraphics({
            image: new Cesium.CallbackProperty(() => { return this.icon }, false),
            scale: new Cesium.CallbackProperty(() => {
                return this.hovered ? HOVER_SCALE_MULTIPLIER : 1.0
            }, false),
            color: new Cesium.CallbackProperty(() => { return this.baseColor.withAlpha(this.opacity) }, false),
            disableDepthTestDistance: Number.POSITIVE_INFINITY,
            show: new Cesium.CallbackProperty(() => { return this.hasPosition() && this.visible }, false),
        });
    }

    drag(screenXY: Cesium.Cartesian2, modifier: KeyModifier): boolean {
        if (modifier != KeyModifier.None && modifier != KeyModifier.Shift)
            return false;

        const scene = this.cesium.scene;

        // Normal by camera if any modifier, else normal by surface
        let normal = modifier == KeyModifier.Shift ?
            Cesium.Cartesian3.subtract(scene.camera.position, this.cartesian, new Cesium.Cartesian3()) :
            scene.globe.ellipsoid.geodeticSurfaceNormal(this.cartesian);

        if (!Cesium.defined(normal))
            return false;

        normal = Cesium.Cartesian3.normalize(normal, normal)

        // Cast ray from camera to plane projected by cartesian and normal
        const ray = scene.camera.getPickRay(screenXY);
        const plane = Cesium.Plane.fromPointNormal(this.cartesian, normal);
        const cartesian = Cesium.IntersectionTests.rayPlane(ray!, plane);
        if (!cartesian)
            return false;
        let newGeodetic = scene.globe.ellipsoid.cartesianToCartographic(cartesian);

        if (modifier == KeyModifier.Shift) {
            const geodetic = scene.globe.ellipsoid.cartesianToCartographic(this.cartesian);
            newGeodetic.latitude = geodetic.latitude;
            newGeodetic.longitude = geodetic.longitude;
        }

        const newCartesian = scene.globe.ellipsoid.cartographicToCartesian(newGeodetic);
        this.draggingListeners.forEach(listener => listener(newCartesian));
        return true;
    }

    icon: string
}

export class PylonEntity extends BasePointEntity {
    constructor(cesium: Cesium.Viewer, width: number) {
        super(cesium);

        this.terrainCartesian = Cesium.Cartesian3.ZERO;
        this.terrainListeners = []; // TODO: useless?

        this.entity.polyline = new Cesium.PolylineGraphics({
            positions: new Cesium.CallbackProperty(() => {
                return [this.cartesian, this.terrainCartesian]
            }, false),
            arcType: Cesium.ArcType.NONE,
            material: new Cesium.PolylineArrowMaterialProperty(
                new Cesium.CallbackProperty(() => { return this.baseColor.withAlpha(this.opacity) }, false)
            ),
            width: width,
            show: new Cesium.CallbackProperty(() => { return this.hasPosition() && this.visible }, false),
        });
    }

    subscribeTerrain(listener: Function) { this.terrainListeners.push(listener); }
    unsubscribeTerrain(listener: Function) { this.terrainListeners = this.terrainListeners.filter(item => item !== listener); }

    setCartesian(cartesian: Cesium.Cartesian3) {
        super.setCartesian(cartesian);

        // Sample terrain position from the ground
        if (this.hasPosition()) {
            const cartographic = Cesium.Cartographic.fromCartesian(this.cartesian);
            const promise = Cesium.sampleTerrainMostDetailed(this.cesium.terrainProvider, [cartographic]);
            promise.then(updatedPositions => {
                this.terrainCartesian = Cesium.Cartographic.toCartesian(cartographic)
                const terrainAltitude = cartographic.height
                this.terrainListeners.forEach(listener => listener(terrainAltitude))
            });
        }
    }

    private terrainCartesian: Cesium.Cartesian3
    protected terrainListeners: Array<Function>
}

export class CircleEntity extends BasePointEntity {
    constructor(cesium: Cesium.Viewer, width: number) {
        super(cesium);

        this.width = width;
        this.radius = undefined;
        this.height = 0.0;

        this.entity.ellipse = new Cesium.EllipseGraphics({
            fill: false,
            height: new Cesium.CallbackProperty(() => { return this.height }, false),
            semiMinorAxis: new Cesium.CallbackProperty(() => { return this.radius }, false),
            semiMajorAxis: new Cesium.CallbackProperty(() => { return this.radius }, false),
            outline: true,
            outlineWidth: new Cesium.CallbackProperty(() => {
                return this.hovered ? this.width * HOVER_SCALE_MULTIPLIER : this.width
            }, false),
            outlineColor: new Cesium.CallbackProperty(() => { return this.baseColor.withAlpha(this.opacity) }, false)
        });
    }

    drag(screenXY: Cesium.Cartesian2, modifier: KeyModifier): boolean {
        if (modifier != KeyModifier.None)
            return false;

        const scene = this.cesium.scene;

        // Normal by camera if any modifier, else normal by surface
        let normal = scene.globe.ellipsoid.geodeticSurfaceNormal(this.cartesian);
        if (!Cesium.defined(normal))
            return false;

        normal = Cesium.Cartesian3.normalize(normal, normal);

        // Cast ray from camera to plane projected by cartesian and normal
        const ray = scene.camera.getPickRay(screenXY);
        const plane = Cesium.Plane.fromPointNormal(this.cartesian, normal);
        const cartesian = Cesium.IntersectionTests.rayPlane(ray!, plane);

        const radius = Cesium.Cartesian3.distance(cartesian, this.cartesian);
        this.draggingListeners.forEach(listener => listener(radius));
        return true;
    }

    setDragging(dragging: boolean) {
        if (this.dragging === dragging)
            return;

        this.dragging = dragging;
        if (!dragging) {
            this.draggedListeners.forEach(listener => listener(this.radius));
        }
    }

    width: number
    radius: number | undefined
    height: number
}

export class ModelEntity extends BasePointEntity {
    constructor(cesium: Cesium.Viewer) {
        super(cesium);

        this.modelUri = "";
        this.silhouetteColor = Cesium.Color.GRAY;

        this.hpr = new Cesium.HeadingPitchRoll(0, 0, 0);

        this.entity.orientation = new Cesium.CallbackProperty(() => {
            return Cesium.Transforms.headingPitchRollQuaternion(this.cartesian, this.hpr);
        }, false);

        this.entity.model = new Cesium.ModelGraphics({
            uri: new Cesium.CallbackProperty(() => { return this.modelUri }, false),
            color: new Cesium.CallbackProperty(() => { return this.baseColor.withAlpha(this.opacity) }, false),
            silhouetteColor: new Cesium.CallbackProperty(() => { return this.silhouetteColor }, false),
            minimumPixelSize: 196,
            maximumScale: 80000,
            colorBlendMode: Cesium.ColorBlendMode.REPLACE,
            silhouetteSize: 2.0
        });
    }

    setHpr(heading: number, pitch: number, roll: number) {
        this.hpr = new Cesium.HeadingPitchRoll(Cesium.Math.toRadians(heading - 90),
                                                Cesium.Math.toRadians(pitch),
                                                Cesium.Math.toRadians(roll));
    }

    private hpr: Cesium.HeadingPitchRoll;

    modelUri: string;
    silhouetteColor: Cesium.Color;
}

export class PathEntity extends BaseEntity {
    constructor(cesium: Cesium.Viewer, length: number) {
        super(cesium);

        this.length = length;
        this.track = [];
    }

    done() { this.clear(); }

    clear() {
        this.track.forEach(entity => this.cesium.entities.remove(entity));
        this.track = [];
    }

    addCartesian(cartesian: Cesium.Cartesian3) {
        const point = this.cesium.entities.add({
            position: cartesian,
            point: {
                pixelSize : 4,
                color: new Cesium.CallbackProperty(() => { return this.baseColor.withAlpha(this.opacity) }, false),
                show: new Cesium.CallbackProperty(() => { return this.visible }, false),
            },
        });
        this.track.push(point);

        // Remove extra points
        if (this.length >= 0) {
            let pointsToClear = this.track.length - this.length;
            if (pointsToClear > 0) {
                for (var i = 0; i < pointsToClear; ++i) {
                    this.cesium.entities.remove(this.track.shift()!);
                }
            }
        }
    }

    setCartesians(cartesians: Array<Cesium.Cartesian3>) {
        this.clear();
        cartesians.forEach(cartesian=> { this.addCartesian(cartesian) });
    }

    private length: number
    private track: Array<Cesium.Entity>
}

export class ProjectionEntity extends BaseEntity {
    constructor(cesium: Cesium.Viewer) {
        super(cesium);

        this.sensorRoot = Cesium.Cartesian3.ZERO;
        this.left = Cesium.Cartesian3.ZERO;
        this.right = Cesium.Cartesian3.ZERO;

        this.distance = 10000;
        this.azimuth = 0;

        this.direction = this.cesium.entities.add({
            polygon: {
                hierarchy: new Cesium.CallbackProperty(() => {
                    let array = []
                    if (this.left !== Cesium.Cartesian3.ZERO)
                        array.push(this.left)
                    if (this.right !== Cesium.Cartesian3.ZERO)
                        array.push(this.right)
                    if (this.sensorRoot !== Cesium.Cartesian3.ZERO)
                        array.push(this.sensorRoot)
                    return new Cesium.PolygonHierarchy(array)
                }, false),
                material: new Cesium.StripeMaterialProperty({
                    evenColor: new Cesium.CallbackProperty(() => { return this.baseColor.withAlpha(0.25) }, false),
                    oddColor: Cesium.Color.TRANSPARENT,
                }),
                stRotation: new Cesium.CallbackProperty(() => { return Cesium.Math.toRadians(this.azimuth) }, false),
                perPositionHeight: true,
                arcType: Cesium.ArcType.GEODESIC
            }
        });
    }

    done() { this.cesium.entities.remove(this.direction); }

    setCartesian(cartesian: Cesium.Cartesian3) { this.sensorRoot = cartesian; }
    setDirection(azimuth: number, elevation: number, fov: number) {
        this.azimuth = azimuth;

        this.left = this._castRayDistanceLimitedBySurface(azimuth - fov / 2, elevation);
        this.right = this._castRayDistanceLimitedBySurface(azimuth + fov / 2, elevation);
    }
    setDistance(distance: number) { this.distance = distance; }

    _castRayDistance(azimuth: number, elevation: number): Cesium.Cartesian3 {
        const hpr = Utils.hprFromAttitude(azimuth, elevation);
        const ray = Utils.castRay(this.sensorRoot, hpr);
        return Cesium.Ray.getPoint(ray, this.distance);
    }

    _castRayToSurface(azimuth: number, elevation: number): Cesium.Cartesian3 {
        const hpr = Utils.hprFromAttitude(azimuth, elevation);
        const ray = Utils.castRay(this.sensorRoot, hpr);
        return this.cesium.scene.globe.pick(ray, this.cesium.scene) || Cesium.Cartesian3.ZERO;
    }

    _castRayDistanceLimitedBySurface(azimuth: number, elevation: number): Cesium.Cartesian3 {
        const cartesian = this._castRayToSurface(azimuth, elevation);
        if (cartesian && cartesian !== Cesium.Cartesian3.ZERO)
            return cartesian;

        return this._castRayDistance(azimuth, elevation);
    }

    protected sensorRoot: Cesium.Cartesian3
    private left: Cesium.Cartesian3
    private right: Cesium.Cartesian3

    private distance: number
    private azimuth: number

    private direction: Cesium.Entity
}

export class RectProjectionEntity extends ProjectionEntity {
    constructor(cesium: Cesium.Viewer) {
        super(cesium);

        this.maxElevation = MAX_ELEVATION_DEFAULT;

        this.topLeft = Cesium.Cartesian3.ZERO;
        this.topRight = Cesium.Cartesian3.ZERO;
        this.bottomRight = Cesium.Cartesian3.ZERO;
        this.bottomLeft = Cesium.Cartesian3.ZERO;

        this.projection = this.cesium.entities.add({
            polygon: {
                hierarchy: new Cesium.CallbackProperty(() => {
                    let array = [];
                    if (this.topLeft !== Cesium.Cartesian3.ZERO)
                        array.push(this.topLeft);
                    if (this.topRight !== Cesium.Cartesian3.ZERO)
                        array.push(this.topRight);
                    if (this.bottomRight !== Cesium.Cartesian3.ZERO)
                        array.push(this.bottomRight);
                    if (this.bottomLeft !== Cesium.Cartesian3.ZERO)
                        array.push(this.bottomLeft);
                    return new Cesium.PolygonHierarchy(array);
                }, false),
                outline: true,
                outlineWidth: 8.0,
                outlineColor: new Cesium.CallbackProperty(() => { return this.baseColor }, false),
                material: new Cesium.ColorMaterialProperty(new Cesium.CallbackProperty(() => {
                    return this.baseColor.withAlpha(0.25)
                }, false)),
                perPositionHeight: true,
                arcType: Cesium.ArcType.GEODESIC
            }
        });
    }

    done() {
        super.done();
        this.cesium.entities.remove(this.projection);
    }

    setProjection(azimuth: number, elevation: number, horizontal_fov: number, vertical_fov: number) {
        this.setDirection(azimuth, elevation, horizontal_fov);

        // Show only pointing lower
        if (elevation < this.maxElevation) {
            this.topLeft = this._castRayToSurface(azimuth - horizontal_fov / 2, elevation - vertical_fov / 2);
            this.topRight = this._castRayToSurface(azimuth + horizontal_fov / 2, elevation - vertical_fov / 2);
            this.bottomRight = this._castRayToSurface(azimuth + horizontal_fov / 2, elevation + vertical_fov / 2);
            this.bottomLeft = this._castRayToSurface(azimuth - horizontal_fov / 2, elevation + vertical_fov / 2);
        } else {
            this.topLeft = Cesium.Cartesian3.ZERO;
            this.topRight = Cesium.Cartesian3.ZERO;
            this.bottomRight = Cesium.Cartesian3.ZERO;
            this.bottomLeft = Cesium.Cartesian3.ZERO;
        }
    }

    maxElevation: number

    topLeft: Cesium.Cartesian3
    topRight: Cesium.Cartesian3
    bottomRight: Cesium.Cartesian3
    bottomLeft: Cesium.Cartesian3

    private projection: Cesium.Entity
}
