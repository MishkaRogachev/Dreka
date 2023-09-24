export const server_root = "http://localhost:45486/"

export async function ping(): Promise<String> {
    const response = await fetch(server_root, {
        method: 'GET',
        mode: 'no-cors',
        credentials: 'include'
    });

    let txt = await response.json()
    console.log(txt)
    return txt
}