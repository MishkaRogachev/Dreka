import type { Mission, MissionStatus } from "$bindings/mission";
import { send_request, default_headers } from "$datasource/rest";

export class MissionService {
    static async downloadMission(missionId: string): Promise<MissionStatus | null> {
        return await send_request("/mission/download/", {
            method: "POST",
            body: JSON.stringify(missionId),
            headers: default_headers
        }) || null;
    }

    static async uploadMission(missionId: string): Promise<MissionStatus | null> {
        return await send_request("/mission/upload/", {
            method: "POST",
            body: JSON.stringify(missionId),
            headers: default_headers
        }) || null;
    }

    static async clearMission(missionId: string): Promise<MissionStatus | null> {
        return await send_request("/mission/clear/", {
            method: "POST",
            body: JSON.stringify(missionId),
            headers: default_headers
        }) || null;
    }

    static async cancelMissionState(missionId: string): Promise<MissionStatus | null> {
        return await send_request("/mission/cancel/", {
            method: "PUT",
            body: JSON.stringify(missionId),
            headers: default_headers
        }) || null;
    }

    static async saveFullMission(mission: Mission): Promise<Mission | null> {
        return await send_request("/mission/save", {
            method: "POST",
            body: JSON.stringify(mission),
            headers: default_headers
        }) || null;
    }

    // TODO: save mission item, remove mission item

    static async getMission(id: string): Promise<Mission | null> {
        return await send_request("/mission/mission/" + id, { method: "GET" }) || null;
    }

    static async getMissions(): Promise<Array<Mission> | null> {
        return await send_request("/mission/missions", { method: "GET" }) || null;
    }
}
