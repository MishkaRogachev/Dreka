import type { ExecuteCommandRequest, CommandExecution } from "$bindings/commands";
import { send_request, default_headers } from "$datasource/rest";

export class CommandService {
    static async executeCommand(request: ExecuteCommandRequest): Promise<string | null> {
        return await send_request("/commands/execute/", {
            method: "POST",
            body: JSON.stringify(request),
            headers: default_headers
        }) || null;
    }

    static async cancelCommand(command_id: string): Promise<string | null> {
        return await send_request("/commands/cancel/", {
            method: "PUT",
            body: JSON.stringify(command_id),
            headers: default_headers
        }) || null;
    }

    static async getCommandExecution(id: string): Promise<CommandExecution | null> {
        return await send_request("/commands/execution/" + id, { method: "GET" }) || null;
    }

    static async getCommandExecutions(): Promise<CommandExecution[] | null> {
        return await send_request("/commands/executions", { method: "GET" }) || null;
    }
}
