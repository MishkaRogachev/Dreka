import type { LinkDescription, LinkStatus } from "$bindings/communication";
import type { VehicleTelemetry } from "$bindings/telemetry";
import type { VehicleDescription, VehicleStatus } from "$bindings/vehicles";
import type { CommandExecution } from "$bindings/commands";
import type { Mission, MissionStatus } from "$bindings/mission";

export interface ServerEvent {
    LinkUpserted?: { link: LinkDescription };
    LinkRemoved?: { link_id: string };
    LinkStatusUpdated?: { status: LinkStatus };
    VehicleUpserted?: { vehicle: VehicleDescription };
    VehicleRemoved?: { vehicle_id: string };
    VehicleStatusUpdated?: { status: VehicleStatus };
    TelemetryUpdated?: { telemetry: VehicleTelemetry };
    CommandExecutionUpserted?: { execution: CommandExecution };
    CommandExecutionRemoved?: { id: string };
    MissionUpserted?: { mission: Mission };
    MissionRemoved?: { mission_id: string };
    MissionStatusUpdated?: { status: MissionStatus };
}