import { writable, get } from 'svelte/store';

const userKey = "user";

export const userPreferences = writable(getPreferences());

export function readPreferences() {
    userPreferences.set(getPreferences())
}

export function savePreferences() {
    let data = JSON.stringify(Object.fromEntries(get(userPreferences)));
    localStorage.setItem(userKey, data);
}

function getPreferences(): Map<string, string> {
    let localData = localStorage.getItem(userKey);
    if (!!localData) {
        let parsed = JSON.parse(localData);
        if (!!parsed) {
            return new Map(Object.entries(parsed));
        }
    }
    return new Map();
}

// Save user preferences every second
let interval = setInterval(() => { savePreferences(); }, 1000);
