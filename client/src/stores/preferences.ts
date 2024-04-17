import { writable, get } from 'svelte/store';

const USER_KEY = "user";
const SAVE_PREFERENCES_INTERVAL = 1000;

export const userPreferences = writable(getPreferences());

export function readPreferences() {
    userPreferences.set(getPreferences())
}

export function savePreferences() {
    let data = JSON.stringify(Object.fromEntries(get(userPreferences)));
    localStorage.setItem(USER_KEY, data);
}

function getPreferences(): Map<string, string> {
    let localData = localStorage.getItem(USER_KEY);
    if (!!localData) {
        let parsed = JSON.parse(localData);
        if (!!parsed) {
            return new Map(Object.entries(parsed));
        }
    }
    return new Map();
}

// TODO: replace with explicit call to savePreferences()
let interval = setInterval(() => { savePreferences(); }, SAVE_PREFERENCES_INTERVAL);
