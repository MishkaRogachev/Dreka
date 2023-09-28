const base_url = "http://127.0.0.1:45486";

export const default_headers = new Headers({
    "Content-Type": "application/json",
    "Access-Control-Allow-Origin": base_url,
    "Access-Control-Allow-Methods": "GET, POST, OPTIONS, PUT, PATCH, DELETE",
    "Access-Control-Allow-Headers": "Content-Type, Authorization",
})

export async function send_request(request: string, init?: RequestInit): Promise<any> {
    try {
        const response = await fetch(base_url + request, init);

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


