
import type { Geodetic, Cartesian } from "$lib/interfaces/common"

export type ClickListener = (geodetic: Geodetic, position: Cartesian) => boolean;

export interface MapInteraction {
    mouseCoordinates: () => Cartesian

    subscribeClick: (listener: ClickListener) => void
    unsubscribeClick: (listener: ClickListener) => void
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