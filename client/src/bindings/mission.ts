import { type Geodetic } from "$bindings/spatial";

export enum MissionRouteItemType {
    Gap = "Gap",
    Waypoint = "Waypoint",
    Takeoff = "Takeoff",
    LandStart = "LandStart",
    Landing = "Landing",
    LoiterTrn = "LoiterTrn",
    LoiterAlt = "LoiterAlt",
    TriggerCam = "TriggerCam",
}

export interface MissionRouteItem {
    type: MissionRouteItemType;
    position?: Geodetic;
    hold?: number;
    pass_radius?: number;
    accept_radius?: number;
    yaw?: number | null;
    pitch?: number;
    abort_altitude?: number | null;
    heading_required?: boolean;
    radius?: number;
    turns?: number;
    clockwise?: boolean;
    distance?: number;
    shutter?: number;
    trigger?: boolean;
}

export interface MissionRoute {
    id: string;
    items: MissionRouteItem[];
}

export interface MissionUpdateState {
    NotActual?: {};
    PrepareDownload?: {};
    Download?: { total: number, progress: number };
    PrepareUpload?: { total: number };
    Upload?: { total: number, progress: number };
    Actual?: { total: number };
    Clearing?: {};
}

export interface MissionProgress {
    current?: number;
    reached: Array<number>;
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
