
import type { Geodetic, Cartesian } from "$lib/interfaces/common"

export enum KeyModifier {
    None,
    Ctrl,
    Shift,
    Option
}

export interface Interactable {
    setDragging: (dragging: boolean) => void
    setHovered: (hovered: boolean) => void

    matchInteraction: (objects: Array<any>) => boolean

    drag: (screenXY: Cartesian, modifier: KeyModifier) => boolean
    click: () => boolean

    isDraggable: () => boolean
}

export interface MapInteraction {
    mouseCoordinates: () => Cartesian

    // listener is Fn(geodetic: IGeodetic) => bool
    subscribeClick: (listener: Function) => void
    unsubscribeClick: (listener: Function) => void
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