import { send_request, server_root, RequestType } from "./common";

export class AppService {
    constructor() {
        this.serverOnline = false
    }

    pingServer() {
        send_request(RequestType.Get, server_root, (response: any) => {
            this.serverOnline = !!response
            console.log(response)
        })
    }

    private serverOnline: boolean
}
