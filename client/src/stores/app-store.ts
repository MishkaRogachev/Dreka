import { readable } from 'svelte/store';

import { AppService } from '$lib/services/app';

export const isServerOnline = readable(false, (set) => {
    const pingInterval = setInterval(() => {
        AppService.pingServer().then((onlineStatus: any) => {
            set(onlineStatus as boolean)
        });
    }, 1000);

    return () => clearInterval(pingInterval)
})
