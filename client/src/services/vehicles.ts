import type { VehicleDescription, VehicleStatus } from "$bindings/vehicles";
import { send_request, default_headers } from "$datasource/rest";

export class VehiclesService {
    static async getVehicles(): Promise<Array<VehicleDescription>> {
        return await send_request("/vehicles", { method: "GET" }) || [];
    }

    static async getVehicleStatus(vehicleId: string): Promise<VehicleStatus | null> {
        return await send_request("/vehicles/status/" + vehicleId, { method: "GET" }) || null;
    }

    static async getVehicleStatuses(vehicleIds: Array<string>): Promise<Array<VehicleStatus>> {
        return await send_request("/vehicles/statuses/" + vehicleIds.join(","), { method: "GET" }) || null;
    }

    static async saveVehicle(vehicle: VehicleDescription): Promise<VehicleDescription | null> {
        return await send_request("/vehicles/save", {
            method: "POST",
            body: JSON.stringify(vehicle),
            headers: default_headers
        }) || null;
    }

    static async removeVehicle(vehicleId: string): Promise<string | null> {
        return await send_request("/vehicles/remove/" + vehicleId, { method: "DELETE" }) || null;
    }
}
