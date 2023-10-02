import type { VehicleDescription } from "$bindings/vehicles";
import { send_request, default_headers } from "$datasource/rest";

export class VehiclesService {
    static async getVehicles(): Promise<Array<VehicleDescription>> {
        const json = await send_request("/vehicles", { method: "GET" });

        if (!!json) {
            return json;
        }

        return [];
    }

    static async addVehicle(vehicle: VehicleDescription): Promise<VehicleDescription | null> {
        let request = !vehicle.id ? "/vehicles/create" : "/vehicles/update"

        const json = await send_request(request, {
            method: "POST",
            body: JSON.stringify(vehicle),
            headers: default_headers
        });

        if (!!json) {
            return json;
        }

        return null;
    }
}
