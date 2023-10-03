import type { VehicleDescription } from "$bindings/vehicles";
import { send_request, default_headers } from "$datasource/rest";

export class VehiclesService {
    static async getVehicles(): Promise<Array<VehicleDescription>> {
        return await send_request("/vehicles", { method: "GET" }) || [];
    }

    static async saveVehicle(vehicle: VehicleDescription): Promise<VehicleDescription | null> {
        let request = !vehicle.id ? "/vehicles/create" : "/vehicles/update"

        return await send_request(request, {
            method: "POST",
            body: JSON.stringify(vehicle),
            headers: default_headers
        }) || null;
    }
}
