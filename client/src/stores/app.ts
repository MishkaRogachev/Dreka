import { writable, get, type Writable } from 'svelte/store';

import type { MapFacade } from '$lib/interfaces/map';

import { userPreferences } from '$stores/preferences';

export enum Theme { Light = "light", Dark = "dark" }

export const theme = writable(getTheme())

export const scales = [0.75, 1.00, 1.25, 1.5, 2.0]
export const scale = writable(getScale())

export const mainMap: Writable<MapFacade | null> = writable(null);

export const dashboardVisible: Writable<boolean>  = writable(true);
export const activeMapPopup: Writable<string>  = writable("");
export const activeDialog: Writable<any> = writable(undefined);

export function closeAllPopups() {
    activeMapPopup.set("");
    activeDialog.set(undefined);
}

export function activateDialog(comonent: any, props: any) {
    activeMapPopup.set("");
    activeDialog.set({ component: comonent, props: props });
}

export function activateMapPopup(idd: string, closeDialog: boolean) {
    if (closeDialog) {
        activeDialog.set(undefined);
    }
    activeMapPopup.set(idd);
}

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
