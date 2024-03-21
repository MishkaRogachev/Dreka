import type { LinkDescription, LinkStatus } from "$bindings/communication";
import { send_request, default_headers } from "$datasource/rest";

export class CommunicationService {
    static async getLinkDescription(linkId: string): Promise<LinkDescription | null> {
        return await send_request("/comm/links/description/" + linkId, { method: "GET" }) || null;
    }

    static async getLinkDescriptions(): Promise<Array<LinkDescription>> {
        return await send_request("/comm/links/descriptions", { method: "GET" }) || [];
    }

    static async getLinkStatus(linkId: string): Promise<LinkStatus | null> {
        return await send_request("/comm/links/status/" + linkId, { method: "GET" }) || null;
    }

    static async getLinkStatuses(): Promise<Array<LinkStatus>> {
        return await send_request("/comm/links/statuses", { method: "GET" }) || [];
    }

    static async saveLinkDescription(link: LinkDescription): Promise<LinkDescription | null> {
        return await send_request("/comm/links/save", {
            method: "POST",
            body: JSON.stringify(link),
            headers: default_headers
        }) || null;
    }

    static async removeLink(linkId: string): Promise<string | null> {
        return await send_request("/comm/links/remove/" + linkId, { method: "DELETE" }) || null;
    }

    static async setLinkEnabled(linkId: string, connected: boolean) {
        await send_request("/comm/links/set_connected/" + linkId, {
            method: "PUT",
            body: JSON.stringify(connected),
            headers: default_headers
        }) || null;
    }
}
