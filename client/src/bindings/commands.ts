import { type Geodetic } from "$bindings/spatial";
import type { VehicleMode } from "$bindings/vehicles";

export interface Command {
    ArmDisarm?: { arm: boolean };
    SetMode?: { mode: VehicleMode };
    SetWaypoint?: { wpt: number };
    SetHome?: { position: Geodetic };

    ReturnToLaunch?: {};
    NavTo?: { position: Geodetic };

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

export interface CommandState {
    Initial?: {};                           // Initial state
    Sent?: { attempt: number };             // Command sent to executor
    Accepted?: {};                          // Command accepted by executor
    Rejected?: {};                          // Command rejected by executor
    Denied?: {};                            // Command denied by executor
    Unsupported?: {};                       // Command unsupported by executor or protocol
    Failed?: {};                            // Command failed to execute by protocol
    InProgress?: { progress: number };      // Command in progress by executor
    Canceled?: {};                          // Command canceled by user
}

export interface CommandExecutor {
    Vehicle?: { vehicle_id: string };
    Payload?: { vehicle_id: string, payload_id: string };
}

export interface ExecuteCommandRequest {
    command: Command,
    executor: CommandExecutor
}

export interface CommandExecution {
    id: string;
    command: Command;
    executor: CommandExecutor;
    state: CommandState;
}
