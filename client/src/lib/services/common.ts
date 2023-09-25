export const server_root = "http://127.0.0.1:45486/";

export enum RequestType {
    Get = "Get",
    Post = "POST"
}

export function send_request(type: RequestType, url: string, callback: Function, timeout: number = 2000) {
    const xhr = new XMLHttpRequest();

    xhr.ontimeout = () => {
        callback(null);
    };

    xhr.onload = () => {
        if (xhr.readyState === 4) {
            if (xhr.status === 200) {
                callback(xhr.response);
            } else {
                callback(null);
            }
        }
    };

    xhr.open(type, url, true);
    xhr.timeout = timeout;
    xhr.send(null);
}
