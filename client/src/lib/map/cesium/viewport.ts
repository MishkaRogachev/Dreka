import type { MapViewport, MapViewportSettings } from '$lib/interfaces/map';
import { type Cartesian, type Geodetic, GeodeticFrame, nullGeodetic } from '$lib/interfaces/common';

import * as Cesium from 'cesium';

export class MapViewportCesium implements MapViewport {
    constructor(cesium: Cesium.Viewer) {
        this.cesium = cesium;
    }

    flyTo(latitude: number, longitude: number, altitude: number, heading: number, pitch: number, duration: number): void {
        this.cesium.camera.flyTo({
            destination: Cesium.Cartesian3.fromDegrees(longitude, latitude, altitude),
            orientation: {
                heading: Cesium.Math.toRadians(heading),
                pitch: Cesium.Math.toRadians(pitch),
                roll: 0.0
            },
            duration: duration
        });
    }

    setView(latitude: number, longitude: number, altitude: number, heading: number, pitch: number): void {
        this.cesium.camera.setView({
            destination: Cesium.Cartesian3.fromDegrees(longitude, latitude, altitude),
            orientation: {
                heading: Cesium.Math.toRadians(heading),
                pitch: Cesium.Math.toRadians(pitch),
                roll: 0.0
            }
        });
    }

    lookTo(heading: number, pitch: number, duration: number): void {
        this.cesium.camera.flyTo({
            destination : this.cesium.camera.position,
            orientation : {
                heading : Cesium.Math.toRadians(heading),
                pitch : Cesium.Math.toRadians(pitch),
                roll: this.cesium.camera.roll
            },
            duration: duration
        });
    }

    zoomIn(amount: number): void {
        this.cesium.camera.zoomIn(amount);
    }

    zoomOut(amount: number): void {
        this.cesium.camera.zoomOut(amount);
    }

    screenXYToGeodetic(point: Cartesian): Geodetic {
        if (!point.x || !point.y) {
            return nullGeodetic;
        }

        const globe = this.cesium.scene.globe;
        const ray = this.cesium.camera.getPickRay(new Cesium.Cartesian2(point.x, point.y));
        if (!ray)
            return nullGeodetic;

        const cartesian = globe.pick(ray, this.cesium.scene);
        if (!cartesian)
            return nullGeodetic;

        const cartographic = globe.ellipsoid.cartesianToCartographic(cartesian);
        return {
            latitude: Cesium.Math.toDegrees(cartographic.latitude),
            longitude: Cesium.Math.toDegrees(cartographic.longitude),
            altitude: cartographic.height,
            frame: GeodeticFrame.Wgs84AboveSeaLevel
        }
    }

    geodeticToScreenXY(geodetic: Geodetic): Cartesian {
        if (!geodetic)
            return { x: NaN, y: NaN };

        const cartographic = Cesium.Cartographic.fromDegrees(geodetic.longitude, geodetic.latitude, geodetic.altitude);
        return Cesium.SceneTransforms.wgs84ToWindowCoordinates(this.cesium.scene, Cesium.Cartographic.toCartesian(cartographic));
    }

    viewportWidth(): number {
        return this.cesium.scene.canvas.clientWidth;
    }

    viewportHeight(): number {
        return this.cesium.scene.canvas.clientHeight;
    }

    heading(): number {
        return Cesium.Math.toDegrees(this.cesium.camera.heading);
    }

    pixelScale(): number {
        // Find the distance between two pixels in the center of the screen.
        const left = this.cesium.camera.getPickRay(new Cesium.Cartesian2((this.viewportWidth() / 2) | 0, this.viewportHeight() / 2));
        const right = this.cesium.camera.getPickRay(new Cesium.Cartesian2(1 + (this.viewportWidth() / 2) | 0, this.viewportHeight() / 2));
        const globe = this.cesium.scene.globe;
        const leftPosition = globe.pick(left!, this.cesium.scene);
        const rightPosition = globe.pick(right!, this.cesium.scene);

        if (Cesium.defined(leftPosition) && Cesium.defined(rightPosition)) {
            const leftCartographic = globe.ellipsoid.cartesianToCartographic(leftPosition);
            const rightCartographic = globe.ellipsoid.cartesianToCartographic(rightPosition);
            const  geodesic = new Cesium.EllipsoidGeodesic();
            geodesic.setEndPoints(leftCartographic, rightCartographic);
            return geodesic.surfaceDistance;
        } else {
            return 0;
        }
    }

    save(): MapViewportSettings {
        const position = this.cesium.camera.positionCartographic;
        return {
            longitude: Cesium.Math.toDegrees(position.longitude),
            latitude: Cesium.Math.toDegrees(position.latitude),
            altitude: position.height,
            heading: Cesium.Math.toDegrees(this.cesium.camera.heading),
            pitch: Cesium.Math.toDegrees(this.cesium.camera.pitch),
        };
    }

    restore(settings: MapViewportSettings) {
        this.cesium.camera.setView({
            destination: Cesium.Cartesian3.fromDegrees(settings.longitude, settings.latitude, settings.altitude),
            orientation: {
                heading: Cesium.Math.toRadians(settings.heading),
                pitch: Cesium.Math.toRadians(settings.pitch),
                roll: 0.0
            }
        })
    }

    private cesium: Cesium.Viewer
}
