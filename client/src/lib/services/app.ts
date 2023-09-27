import { send_request, server_root } from "../datasource/rest";

export class AppService {
    static async pingServer(): Promise<boolean> {
        const response = await send_request(server_root);
        return response == "ok";
    }
}
