
export class Preferences {
    write(key: string, value: string): void {
        window.localStorage.setItem(key, value);
    }

    read(key: string): string | null {
        return window.localStorage.getItem(key);
    }
}

export const preferences = new Preferences()
