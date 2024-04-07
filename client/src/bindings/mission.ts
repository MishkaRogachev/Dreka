import { type Geodetic } from "$bindings/spatial";

export interface MissionRouteItem {
    Gap?: {},
    Waypoint?: { position: Geodetic, hold: number, pass_radius: number, accept_radius: number, yaw?: number };
    Takeoff?: { position: Geodetic, pitch: number, yaw?: number };
    LandStart?: {};
    Landing?: { position: Geodetic, abort_altitude?: number, yaw?: number };
    LoiterTrn?: { position: Geodetic, heading_required: boolean, radius: number, turns: number, clockwise: boolean };
    LoiterAlt?: { position: Geodetic, heading_required: boolean, radius: number, clockwise: boolean };
    TriggerCam?: { distance: number, shutter: number, trigger: boolean };
}

export interface MissionRoute {
    id: string;
    items: MissionRouteItem[];
}

export interface MissionUpdateState {
    NotActual?: {};
    PrepareDownload?: {};
    Download?: { total: number, progress: number };
    Upload?: { total: number, progress: number };
    Actual?: { total: number };
    Clearing?: {};
}

export interface MissionProgress {
    OnHold?: {};
    InProgress?: { current: number, passed: Array<number>; };
    Finished?: { passed: Array<number>; };
}

export interface MissionStatus {
    id: string;
    state: MissionUpdateState;
    progress: MissionProgress;
}

export interface Mission {
    id: string;
    vehicle_id: string;
    route: MissionRoute;
    status: MissionStatus;
}
