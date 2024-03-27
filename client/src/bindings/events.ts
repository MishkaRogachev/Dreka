import type { LinkDescription, LinkStatus } from "$bindings/communication";
import type { VehicleTelemetry } from "$bindings/telemetry";
import type { VehicleDescription, VehicleStatus } from "$bindings/vehicles";
import type { VehicleCommandState } from "$bindings/commands";

export interface ServerEvent {
    LinkUpdated?: { link: LinkDescription };
    LinkRemoved?: { link_id: string };
    LinkStatusUpdated?: { status: LinkStatus };
    VehicleUpdated?: { vehicle: VehicleDescription };
    VehicleRemoved?: { vehicle_id: string };
    VehicleStatusUpdated?: { status: VehicleStatus };
    TelemetryUpdated?: { telemetry: VehicleTelemetry };
    CommandUpdated?: { command: VehicleCommandState };
    CommandRemoved?: { command_id: string };
}