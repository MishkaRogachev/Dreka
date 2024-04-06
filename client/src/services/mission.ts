import type { Mission, MissionStatus } from "$bindings/mission";
import { send_request, default_headers } from "$datasource/rest";

export class MissionService {
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

    // TODO: mission route operations

    static async getMission(id: string): Promise<Mission | null> {
        return await send_request("/missions/mission/" + id, { method: "GET" }) || null;
    }

    static async getMissions(): Promise<Array<Mission> | null> {
        return await send_request("/missions/missions", { method: "GET" }) || null;
    }
}
