const REST_URL = "http://127.0.0.1:45486";

export const default_headers = new Headers({
    "Content-Type": "application/json",
    "Access-Control-Allow-Origin": REST_URL,
    "Access-Control-Allow-Methods": "GET, POST, OPTIONS, PUT, PATCH, DELETE",
    "Access-Control-Allow-Headers": "Content-Type, Authorization",
})

export async function send_request(request: string, init?: RequestInit): Promise<any> {
    try {
        const response = await fetch(REST_URL + request, init);

        if (!response.ok) {
            throw new Error('Network response was not ok');
        }

        const data = await response.json();
        return data;
    } catch (error) {
        // console.error(error);
        return null
    }
}


