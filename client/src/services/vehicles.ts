import type { VehicleDescription, VehicleStatus } from "$bindings/vehicles";
import { send_request, default_headers } from "$datasource/rest";

export class VehiclesService {
    static async getVehicleDescription(vehicleId: string): Promise<VehicleDescription | null> {
        return await send_request("/vehicles/description/" + vehicleId, { method: "GET" }) || null;
    }

    static async getVehicleDescriptions(): Promise<Array<VehicleDescription>> {
        return await send_request("/vehicles/descriptions", { method: "GET" }) || [];
    }

    static async getVehicleStatus(vehicleId: string): Promise<VehicleStatus | null> {
        return await send_request("/vehicles/status/" + vehicleId, { method: "GET" }) || null;
    }

    static async getVehicleStatuses(): Promise<Array<VehicleStatus>> {
        return await send_request("/vehicles/statuses", { method: "GET" }) || [];
    }

    static async saveVehicleDescription(vehicle: VehicleDescription): Promise<VehicleDescription | null> {
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
