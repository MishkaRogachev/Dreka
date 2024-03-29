import { type Geodetic } from "$bindings/spatial";

export interface VehicleCommand {
    ArmDisarm?: { arm: boolean };
    SetMode?: { mode: string };
    SetWaypoint?: { wp: number };
    ReturnToLaunch?: {};
    GoTo?: { wp: number };
    NavTo?: { position: Geodetic };
    SetReturn?: { position: Geodetic };
    SetAltitude?: { altitude: number };
    SetLoiterRadius?: { radius: number };
    CalibrateAirspeed?: {};
    CalibrateReferencePressure?: {};
    CalibrateTemperature?: {};
    SetAirSpeed?: { value: number };
    SetGroundSpeed?: { value: number };
    SetThrottle?: { value: number };
    ManualControl?: { pitch: number; roll: number; yaw: number; thrust: number };
    SetServo?: { channel: number; value: number };
    OverrideServos?: { servos: { [channel: number]: number } };
    Takeoff?: { altitude: number };
    GoAround?: {};
}

export enum CommandState {
    Initial = 'Initial',
    Sent = 'Sent',
    Accepted = 'Accepted',
    Rejected = 'Rejected',
    Denied = 'Denied',
    Unsupported = 'Unsupported',
    Failed = 'Failed',
    InProgress = 'InProgress',
    Canceled = 'Canceled'
}

export interface VehicleCommandState {
    id: string;
    vehicle_id: string;
    attempt: number;
    state: CommandState;
}
