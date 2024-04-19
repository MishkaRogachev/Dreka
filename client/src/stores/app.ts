import { readable, writable, get, type Writable } from 'svelte/store';

import type { MapFacade } from '$lib/interfaces/map';

import { AppService } from '$services/app';
import { userPreferences } from '$stores/preferences';

export enum Theme { Light = "light", Dark = "dark" }

export const theme = writable(getTheme())

export const scales = [0.75, 1.00, 1.25, 1.5, 2.0]
export const scale = writable(getScale())

export const mainMap: Writable<MapFacade | null> = writable(null);

export const dashboardVisible = writable(true);
export const activeMapPopup = writable("");

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

// TODO: replace with a web socket state check
export const isServerOnline = readable(false, (set) => {
    const pingInterval = setInterval(() => {
        AppService.pingServer().then((onlineStatus: any) => {
            set(onlineStatus as boolean);
        });
    }, 1000);

    return () => clearInterval(pingInterval);
})
