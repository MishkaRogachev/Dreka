import { readable, writable, get } from 'svelte/store';

import { AppService } from '$services/app';
import { userPreferences } from '$stores/preferences';

export enum Page {
    Flight = "Flight",
    Communication = "Communication",
    About = "About"
}

export enum Theme {
    Light = "light",
    Dark = "dark",
}

export const pages = [Page.Flight, Page.Communication, Page.About];
export const currentPage = writable(Page.Flight);

export const theme = writable(getTheme())

export const scale = writable(getScale())
export const scales = [0.75, 1.00, 1.25, 1.5, 2.0]

function getTheme(): Theme {
    let themeValue = get(userPreferences).get("ui/theme");
    if (!themeValue) {
        //@ts-ignore
        themeValue = document.querySelector('html').getAttribute('data-theme');
    }
    return themeValue as Theme || Theme.Dark;
}

function applyTheme(theme: Theme) {
    //@ts-ignore
    document.querySelector('html').setAttribute('data-theme', theme);
}

function getScale(): number {
    let scaleValue = parseFloat(get(userPreferences).get("ui/scale") || "");
    if (!scaleValue) {
        scaleValue = 1.0;
    }
    return scaleValue
}

function applyScale(scale: number) {
    //@ts-ignore
    document.body.style.zoom = scale
}

theme.subscribe(themeValue => {
    applyTheme(themeValue)
    get(userPreferences).set("ui/theme", themeValue);
});

scale.subscribe(scaleValue => {
    applyScale(scaleValue)
    get(userPreferences).set("ui/scale", scaleValue.toString());
});

export const isServerOnline = readable(false, (set) => {
    const pingInterval = setInterval(() => {
        AppService.pingServer().then((onlineStatus: any) => {
            set(onlineStatus as boolean)
        });
    }, 1000);

    return () => clearInterval(pingInterval)
})
