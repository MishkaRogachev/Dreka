import type { LinkDescription } from "$bindings/links";
import { send_request, default_headers } from "$datasource/rest";

export class LinksService {
    static async getLinks(): Promise<Array<LinkDescription>> {
        const json = await send_request("/links", { method: "GET" });

        if (!!json) {
            return json;
        }

        return [];
    }

    static async saveLink(link: LinkDescription): Promise<LinkDescription | null> {
        let request = !link.id ? "/links/create" : "/links/update"

        const json = await send_request(request, {
            method: "POST",
            body: JSON.stringify(link),
            headers: default_headers
        });

        if (!!json) {
            return json;
        }

        return null;
    }
}
