import i18next from "i18next";
import { get, writable } from 'svelte/store';
import { createI18nStore } from "svelte-i18next";

import { userPreferences } from '$stores/preferences';

import { degreesToDmsString } from "$lib/common/formats";

import en_us from "$translations/en_us.json"
import ru_ru from "$translations/ru_ru.json"
import tr_tr from "$translations/tr_tr.json"
import uk_ua from "$translations/uk_ua.json"
import type { Geodetic } from "$bindings/spatial";

export const locales = ["en_us", "ru_ru", "tr_tr", "uk_ua"]
export const locale = writable(getLocale())

i18next.init({
    lng: "uk_ua",
    resources: {
        "en_us" : en_us,
        "ru_ru": ru_ru,
        "tr_tr": tr_tr,
        "uk_ua": uk_ua
    },
    interpolation: {
        escapeValue: false, // not needed for svelte as it escapes by default
    }
});

function getLocale(): string {
    return get(userPreferences).get("ui/locale") || "en_us";
}

locale.subscribe(localeValue => {
    i18next.changeLanguage(localeValue)
    get(userPreferences).set("ui/locale", localeValue.toString());
});

i18next.changeLanguage

export let i18n = createI18nStore(i18next);

function dmsFormat(): boolean {
    return parseInt(get(userPreferences).get("ui/coordinates_dms") || "1") != 0;
}

function setDmsFormat(value: boolean) {
    get(userPreferences).set("ui/coordinates_dms", value ? "1" : "0");
}

export function formatGeodeticCoordinates(position: Geodetic | undefined): Array<string> {
    const dms = dmsFormat()
    const latitude = position ? (dms ? degreesToDmsString(position.latitude, false)
                : position.latitude.toFixed(6)) : get(i18n).t("N/A")
    const longitude = position ? (dms ? degreesToDmsString(position.longitude, true)
                    : position.longitude.toFixed(6)) : get(i18n).t("N/A")
    return [latitude, longitude]
}