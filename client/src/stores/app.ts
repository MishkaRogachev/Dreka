import { readable, writable } from 'svelte/store';

import { AppService } from '$services/app';

export enum Pages {
    Flight = "Flight",
    Communication = "Communication",
    About = "About"
}

export const pages = [Pages.Flight, Pages.Communication, Pages.About];
export const currentPage = writable(Pages.Flight);

export const isServerOnline = readable(false, (set) => {
    const pingInterval = setInterval(() => {
        AppService.pingServer().then((onlineStatus: any) => {
            set(onlineStatus as boolean)
        });
    }, 1000);

    return () => clearInterval(pingInterval)
})


