
export interface Cartesian {
    x: number
    y: number
}

export enum GeodeticFrame {
    None = "None",
    Wgs84RelativeHome = "Wgs84RelativeHome",
    Wgs84AboveSeaLevel = "Wgs84AboveSeaLevel",
    Wgs84AboveTerrain = "Wgs84AboveTerrain"
}

export interface Geodetic {
    latitude: number,
    longitude: number,
    altitude: number,
    frame: GeodeticFrame
}

export const nullGeodetic = { latitude: NaN, longitude: NaN, altitude: NaN, frame: GeodeticFrame.None }
