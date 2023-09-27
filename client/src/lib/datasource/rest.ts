export const server_root = "http://127.0.0.1:45486/";

export async function send_request(url: string): Promise<any> {
    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        const data = await response.json();
        return data;
    } catch (error) {
        console.error(error);
        return {}
    }
}
