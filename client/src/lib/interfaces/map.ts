
import type { MissionRouteItem } from "$bindings/mission";
import type { Geodetic, Cartesian } from "$bindings/spatial";

export type ClickListener = (geodetic: Geodetic, position: Cartesian) => boolean;

export interface MapInteraction {
    mouseCoordinates: () => Cartesian

    subscribeClick: (listener: ClickListener) => void
    unsubscribeClick: (listener: ClickListener) => void
}

export interface MapViewportSettings {
    latitude: number,
    longitude: number,
    altitude: number,
    heading: number,
    pitch: number
}

export interface MapViewport {
    flyTo: (latitude: number, longitude: number, altitude: number, heading: number, pitch: number, duration: number) => void
    setView: (latitude: number, longitude: number, altitude: number, heading: number, pitch: number) => void
    lookTo: (heading: number, pitch: number, duration: number) => void
    zoomIn: (amount: number) => void
    zoomOut: (amount: number) => void

    heading: () => number
    pixelScale: () => number

    viewportWidth: () => number
    viewportHeight: () => number

    screenXYToGeodetic: (point: Cartesian) => Geodetic
    geodeticToScreenXY: (geodetic: Geodetic) => Cartesian

    save: () => MapViewportSettings
    restore: (settings: MapViewportSettings) => void

    subscribe: (listener: Function) => void
    unsubscribe: (listener: Function) => void
}

export interface MapRuler {
    setEnabled: (enabled: boolean) => void
    enabled: () => boolean

    clear: () => void
    distance: () => number
}

export interface MapGraticule {
    setEnabled: (enabled: boolean) => void
    enabled: () => boolean
}

export interface ImageryLayer {
    name: string,
    source: string,
    opacity: number,
    visibility: boolean,
    index: number
}

export interface MapLayers {
    imageryLayers: () => Array<ImageryLayer>

    addImageryLayers: (layers: ImageryLayer[]) => Promise<void>
    addImageryLayer: (layer: ImageryLayer) => Promise<void>
    updateImageryLayer: (layer: ImageryLayer) => Promise<void>
    raiseImageryLayer: (layer: ImageryLayer) => Promise<void>
    lowerImageryLayer: (layer: ImageryLayer) => Promise<void>
    removeImageryLayer: (layer: ImageryLayer) => Promise<void>
    resetImageryLayers: () => Promise<void>
}

export enum MapMissionRouteEvent {
    Changed,
    Activated,
    Drag,
    Removed
}
export interface MapMissionRoute {
    subscribe: (event: MapMissionRouteEvent, listener: (item: MissionRouteItem, index: number) => void) => void
}