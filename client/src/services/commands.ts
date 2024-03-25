import type { VehicleCommand, VehicleCommandState } from "$bindings/commands";
import { send_request, default_headers } from "$datasource/rest";

export class CommandService {
    static async executeCommand(vehicleId: string, command: VehicleCommand): Promise<VehicleCommandState | null> {
        return await send_request("/commands/exec/" + vehicleId, {
            method: "POST",
            body: JSON.stringify(command),
            headers: default_headers
        }) || null;
    }

    static async stopCommand(commandId: string): Promise<VehicleCommandState | null> {
        return await send_request("/commands/stop/" + commandId, { method: "PUT" }) || null;
    }

    static async getCommand(commandId: string): Promise<VehicleCommandState | null> {
        return await send_request("/command/" + commandId, { method: "GET" }) || null;
    }

    static async getCommands(): Promise<VehicleCommandState[] | null> {
        return await send_request("/commands", { method: "GET" }) || null;
    }
}
