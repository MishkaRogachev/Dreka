import * as Cesium from 'cesium';

import { type Geodetic, GeodeticFrame } from '$bindings/spatial';

export function intermediate(first: Cesium.Cartesian3, second: Cesium.Cartesian3): Cesium.Cartesian3 {
    const scratch = new Cesium.Cartesian3();
    const difference = Cesium.Cartesian3.subtract(first, second, scratch);
    const distance = -0.5 * Cesium.Cartesian3.magnitude(difference);
    const direction = Cesium.Cartesian3.normalize(difference, scratch);

    return Cesium.Cartesian3.add(first, Cesium.Cartesian3.multiplyByScalar(direction, distance, scratch), scratch);
}

export function hprFromAttitude(heading: number, pitch: number = 0, roll: number = 0): Cesium.HeadingPitchRoll {
    return new Cesium.HeadingPitchRoll(Cesium.Math.toRadians(heading - 90), Cesium.Math.toRadians(pitch), Cesium.Math.toRadians(roll));
}

export function hprFrom2Points(origin: Cesium.Cartesian3, target: Cesium.Cartesian3): Cesium.HeadingPitchRoll {
    const transform = Cesium.Transforms.eastNorthUpToFixedFrame(origin);
    const posVector = Cesium.Cartesian3.subtract(target, origin, new Cesium.Cartesian3());
    const vector = Cesium.Matrix4.multiplyByPointAsVector(Cesium.Matrix4.inverse(transform, new Cesium.Matrix4()), posVector, new Cesium.Cartesian3());
    const direction = Cesium.Cartesian3.normalize(vector, new Cesium.Cartesian3());

    const heading = -1 * Math.atan2(direction.y, direction.x);
    const pitch = Cesium.Math.PI_OVER_TWO - Cesium.Math.acosClamped(direction.z);
    return new Cesium.HeadingPitchRoll(heading, pitch, 0); //  Cesium.Math.toDegrees(Cesium.Math.TWO_PI-Cesium.Math.zeroToTwoPi(heading))
}

export function directionByVector(position: Cesium.Cartesian3, hpr: Cesium.HeadingPitchRoll, vector: Cesium.Cartesian3): Cesium.Cartesian3 {
    const referenceFrame = Cesium.Transforms.headingPitchRollQuaternion(position, hpr);
    const rotationMatrix =  Cesium.Matrix3.fromQuaternion(referenceFrame, new Cesium.Matrix3());
    return Cesium.Matrix3.multiplyByVector(rotationMatrix, vector, new Cesium.Cartesian3());
}

export function offsetted(position: Cesium.Cartesian3, hpr: Cesium.HeadingPitchRoll, distance: number): Cesium.Cartesian3 {
    const direction = directionByVector(position, hpr, new Cesium.Cartesian3(distance, 0, 0));
    return Cesium.Cartesian3.add(position, direction, new Cesium.Cartesian3());
}

export function castRay(position: Cesium.Cartesian3, hpr: Cesium.HeadingPitchRoll): Cesium.Ray {
    const direction = directionByVector(position, hpr, Cesium.Cartesian3.UNIT_X);

    return new Cesium.Ray(position, direction);
}

// TODO: home and terrain altitudes fix
export function cartesianFromGeodetic(geodetic: Geodetic, homeAltitude: 0): Cesium.Cartesian3 {
    if (!geodetic || (isNaN(geodetic.latitude) && isNaN(geodetic.longitude)))
        return Cesium.Cartesian3.ZERO;

    let altitude;
    switch (geodetic.frame) {
    case GeodeticFrame.Wgs84RelativeHome:
        altitude = geodetic.altitude + homeAltitude;
        break;
    case GeodeticFrame.Wgs84AboveSeaLevel:
        altitude = geodetic.altitude;
        break;
    case GeodeticFrame.Wgs84AboveTerrain:
        altitude = geodetic.altitude;
        break;
    case GeodeticFrame.None: // no breack
    default:
        altitude = 0;
        break;
    }
    return Cesium.Cartesian3.fromDegrees(geodetic.longitude, geodetic.latitude, altitude);
}