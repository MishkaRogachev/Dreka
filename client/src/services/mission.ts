import type { Mission, MissionRouteItem } from "$bindings/mission";
import { send_request, default_headers } from "$datasource/rest";

export class MissionService {
    static async createMission(vehicleId: string): Promise<Mission | null> {
        return await send_request("/missions/create", {
            method: "POST",
            body: JSON.stringify(vehicleId),
            headers: default_headers
        }) || null;
    }

    static async setRouteItem(mission_id: string, item: MissionRouteItem, index: number):
        Promise<Array<[number, MissionRouteItem]> | null>
    {
        return await send_request("/missions/" + mission_id + "/set_route_item/" + index, {
            method: "POST",
            body: JSON.stringify(item),
            headers: default_headers
        }) || null;
    }

    // TODO: remove route item

    static async downloadMission(missionId: string): Promise<string | null> {
        return await send_request("/missions/download/" + missionId, { method: "PUT" }) || null;
    }

    static async uploadMission(missionId: string): Promise<string | null> {
        return await send_request("/missions/upload/" + missionId, { method: "PUT" }) || null;
    }

    static async clearMission(missionId: string): Promise<string | null> {
        return await send_request("/missions/clear/" + missionId, { method: "DELETE" }) || null;
    }

    static async cancelMissionState(missionId: string): Promise<string | null> {
        return await send_request("/missions/cancel/" + missionId, { method: "PUT" }) || null;
    }

    static async getMission(id: string): Promise<Mission | null> {
        return await send_request("/missions/mission/" + id, { method: "GET" }) || null;
    }

    static async getMissions(): Promise<Array<Mission> | null> {
        return await send_request("/missions/missions", { method: "GET" }) || null;
    }
}
