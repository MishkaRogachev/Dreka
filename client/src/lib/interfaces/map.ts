
import type { MissionProgress, MissionRoute, MissionRouteItem } from "$bindings/mission";
import type { Geodetic, Cartesian } from "$bindings/spatial";
import type { Flight, Navigation } from "$bindings/telemetry";
import type { VehicleDescription, VehicleStatus } from "$bindings/vehicles";

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
    // TODO: use geodetic here
    resetView: () => void
    setView: (latitude: number, longitude: number, altitude: number, heading: number, pitch: number) => void
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
    isEnabled: () => boolean

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
    allImageryLayers: () => Array<ImageryLayer>

    addImageryLayers: (layers: ImageryLayer[]) => Promise<void>
    addImageryLayer: (layer: ImageryLayer) => Promise<void>
    updateImageryLayer: (layer: ImageryLayer) => Promise<void>
    raiseImageryLayer: (layer: ImageryLayer) => Promise<void>
    lowerImageryLayer: (layer: ImageryLayer) => Promise<void>
    removeImageryLayer: (layer: ImageryLayer) => Promise<void>
    resetImageryLayers: () => Promise<void>
}

export interface MapVehicle {
    centerOnMap: () => void
    setTracking: (tracking: boolean) => void

    updateFromDescription: (description: VehicleDescription) => void
    updateFromStatus: (status: VehicleStatus | undefined) => void
    updateFromFlight: (flight: Flight) => void
    updateFromNavigation: (navigation: Navigation) => void
}

export interface MapVehiclesEvent {
    ActivateVehicle?: { vehicleId: string },
    VehicleHovered?: { vehicleId: string, hovered: boolean },
    HomeHovered?: { vehicleId: string, hovered: boolean },
    TargetHovered?: { vehicleId: string, hovered: boolean },
    HomePositionOrdered?: { vehicleId: string, position: Geodetic },
    TargetPositionOrdered?: { vehicleId: string, position: Geodetic },
}
export type MapVehiclesEventListener = (event: MapVehiclesEvent) => void;

export interface MapVehicles {
    done: () => void

    subscribe: (listener: MapVehiclesEventListener) => void
    unsubscribe: (listener: MapVehiclesEventListener) => void

    setSelectedVehicle: (vehicleId: string) => void
    addVehicle: (vehicleId: string) => MapVehicle
    removeVehicle: (vehicleId: string) => void

    vehicle: (vehicleId: string) => MapVehicle | undefined
    allVehicles: () => Array<MapVehicle>
    vehicleIds: () => Array<string>
}

export interface MapMissionRoute {
    fitOnMap: () => void
    centerOnMap: (index: number) => void

    updateFromRoute: (route: MissionRoute) => void
    updateFromProgress: (progress: MissionProgress, inMissionMode: boolean) => void
    setHomeAltitude: (altitude: number) => void
}

// TODO: add map mission type to aggregate route, fence and rally points

export interface MapMissionsEvent {
    InvokeWaypointMenu?: { missionId: string, item: MissionRouteItem, index: number },
    Hovered?: { missionId: string, item: MissionRouteItem, index: number },
    Exited?: {},
    ChangesOrdered?: { missionId: string, item: MissionRouteItem, index: number },
    WaypointDragged?: { missionId: string, item: MissionRouteItem, index: number, position: Geodetic },
}
export type MapMissionsEventListener = (event: MapMissionsEvent) => void;

export interface MapMissions {
    done: () => void

    subscribe: (listener: MapMissionsEventListener) => void
    unsubscribe: (listener: MapMissionsEventListener) => void

    setSelectedMission: (missionId: string) => void

    addMission: (missionId: string) => MapMissionRoute
    removeMission: (missionId: string) => void

    mission: (missionId: string) => MapMissionRoute | undefined
    allMissions: () => Array<MapMissionRoute>
    missionIds: () => Array<string>
}

export interface MapFacade {
    interaction: MapInteraction
    viewport: MapViewport
    ruler: MapRuler
    graticule: MapGraticule
    layers: MapLayers
    vehicles: MapVehicles
    missions: MapMissions

    calcDistance: (from?: Geodetic, to?: Geodetic) => number | undefined
}
