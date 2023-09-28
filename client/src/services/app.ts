import { send_request } from "$datasource/rest";

export class AppService {
    static async pingServer(): Promise<boolean> {
        const response = await send_request("/", { method: "GET" });
        return response == "ok";
    }
}
