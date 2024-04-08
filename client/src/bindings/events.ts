import type { LinkDescription, LinkStatus } from "$bindings/communication";
import type { VehicleTelemetry } from "$bindings/telemetry";
import type { VehicleDescription, VehicleStatus } from "$bindings/vehicles";
import type { CommandExecution } from "$bindings/commands";
import type { Mission, MissionStatus, MissionRoute, MissionRouteItem } from "$bindings/mission";

export interface ServerEvent {
    // Communication
    LinkUpserted?: { link: LinkDescription };
    LinkRemoved?: { link_id: string };
    LinkStatusUpdated?: { status: LinkStatus };

    // Vehicles
    VehicleUpserted?: { vehicle: VehicleDescription };
    VehicleRemoved?: { vehicle_id: string };
    VehicleStatusUpdated?: { status: VehicleStatus };

    // Telemetry
    TelemetryUpdated?: { telemetry: VehicleTelemetry };

    // Commands
    CommandExecutionUpserted?: { execution: CommandExecution };
    CommandExecutionRemoved?: { id: string };

    // Missions
    MissionUpserted?: { mission: Mission };
    MissionRemoved?: { mission_id: string };
    MissionStatusUpdated?: { status: MissionStatus };
    MissionRouteUpdated?: { route: MissionRoute };
    MissionRouteItemUpserted?: { mission_id: string, index: number, item: MissionRouteItem };
    MissionRouteItemRemoved?: { mission_id: string, index: number };
}
