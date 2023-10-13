import { readable, writable, get } from 'svelte/store';

import { AppService } from '$services/app';
import { userPreferences } from '$stores/preferences';

export enum Pages {
    Flight = "Flight",
    Communication = "Communication",
    About = "About"
}

export enum Themes {
    Light = "light",
    Dark = "dark",
}

export const pages = [Pages.Flight, Pages.Communication, Pages.About];
export const currentPage = writable(Pages.Flight);

export const theme = writable(getTheme())

function getTheme(): Themes {
    let themeValue = get(userPreferences).get("ui/theme");
    if (!themeValue) {
        //@ts-ignore
        themeValue = window.document.querySelector('html').getAttribute('data-theme') || ""
    } else {
        //@ts-ignore
        window.document.querySelector('html').setAttribute('data-theme', themeValue);
    }
    return themeValue as Themes || Themes.Dark
}

theme.subscribe(themeValue => {
    //@ts-ignore
    window.document.querySelector('html').setAttribute('data-theme', themeValue);
    get(userPreferences).set("ui/theme", themeValue);
});

export const isServerOnline = readable(false, (set) => {
    const pingInterval = setInterval(() => {
        AppService.pingServer().then((onlineStatus: any) => {
            set(onlineStatus as boolean)
        });
    }, 1000);

    return () => clearInterval(pingInterval)
})


