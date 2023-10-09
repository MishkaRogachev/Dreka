import type { LinkDescription, LinkStatus } from "$bindings/communication";
import { send_request, default_headers } from "$datasource/rest";

export class CommunicationService {
    static async getLinks(): Promise<Array<LinkDescription>> {
        return await send_request("/comm/links", { method: "GET" }) || [];
    }

    static async getLinkStatus(linkId: string): Promise<LinkStatus | null> {
        return await send_request("/comm/links/status/" + linkId, { method: "GET" }) || null;
    }

    static async saveLink(link: LinkDescription): Promise<LinkDescription | null> {
        let request = !link.id ? "/comm/links/create" : "/comm/links/update"

        return await send_request(request, {
            method: "POST",
            body: JSON.stringify(link),
            headers: default_headers
        }) || null;
    }
}
