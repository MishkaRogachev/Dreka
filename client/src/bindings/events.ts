import type { LinkDescription, LinkStatus } from "$bindings/communication";
import type { VehicleTelemetry } from "$bindings/telemetry";
import type { VehicleDescription, VehicleStatus } from "$bindings/vehicles";
import type { CommandExecution } from "$bindings/commands";
import type { Mission, MissionStatus } from "$bindings/mission";

export interface ServerEvent {
    LinkUpdated?: { link: LinkDescription };
    LinkRemoved?: { link_id: string };
    LinkStatusUpdated?: { status: LinkStatus };
    VehicleUpdated?: { vehicle: VehicleDescription };
    VehicleRemoved?: { vehicle_id: string };
    VehicleStatusUpdated?: { status: VehicleStatus };
    TelemetryUpdated?: { telemetry: VehicleTelemetry };
    CommandExecutionUpdated?: { execution: CommandExecution };
    CommandExecutionRemoved?: { id: string };
    MissionUpdated?: { mission: Mission };
    MissionRemoved?: { mission_id: string };
    MissionStatusUpdated?: { status: MissionStatus };
}