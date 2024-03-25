import i18next from "i18next";
import { get, writable } from 'svelte/store';
import { createI18nStore } from "svelte-i18next";

import { userPreferences } from '$stores/preferences';

import en_us from "$translations/en_us.json"
import ru_ru from "$translations/ru_ru.json"
import tr_tr from "$translations/tr_tr.json"
import uk_ua from "$translations/uk_ua.json"

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
