import type { FlightData, SnsData } from "$bindings/telemetry";
import { send_request } from "$datasource/rest";

export class TelemetryService {
    static async getVehicleFlightData(vehicleId: string): Promise<FlightData | null> {
        return await send_request("/telemetry/flight/" + vehicleId, { method: "GET" }) || null;
    }

    static async getVehicleSnsData(vehicleId: string): Promise<SnsData | null> {
        return await send_request("/telemetry/sns/" + vehicleId, { method: "GET" }) || null;
    }
}
