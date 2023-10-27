import { readable, writable, get } from 'svelte/store';

import { type LinkDescription, type LinkStatus } from '$bindings/communication';
import { CommunicationService } from '$services/communication';

export const linkDescriptions = function () {
    let interval: NodeJS.Timeout;

    const store = writable(new Map<string, LinkDescription>(), (set, _) => {
        interval = setInterval(async () => {
            let new_link = new Map<string, LinkDescription>();
            for (const link of await CommunicationService.getLinks()) {
                if (link.id) {
                    new_link.set(link.id, link);
                }
                store.set(new_link);
            }
        }, 1000); // Refresh link descriptions every second
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        link: (linkId: string) => get(store).get(linkId),
        linkIds: () => Array.from(get(store).keys()),
        links: () => get(store).values(),
        saveLink: async (link: LinkDescription) => {
            let linkBack = await CommunicationService.saveLink(link);
            store.update(links => {
                if (linkBack && linkBack.id) {
                    links.set(linkBack.id, linkBack);
                }
                return links;
            })
            return linkBack;
        },
        removeLink: async (linkId: string) => {
            let linkIdBack = await CommunicationService.removeLink(linkId);
            store.update(links => {
                if (linkIdBack) {
                    links.delete(linkIdBack);
                }
                return links;
            })
        },
        setLinkConnected: async (linkId: string, connected: boolean) => {
            await CommunicationService.setLinkConnected(linkId, connected);
        },
        kill: () => clearInterval(interval)
    }
} ()

export const linkStatuses = function () {
    let interval: NodeJS.Timeout;

    const store = readable(new Map<string, LinkStatus>(), (set, _) => {
        interval = setInterval(async () => {
            let new_statuses = new Map<string, LinkStatus>()
            for (const status of await CommunicationService.getLinkStatuses(linkDescriptions.linkIds())) {
                new_statuses.set(status.id, status)
            }
            set(new_statuses)
        }, 500);
    }); // Refresh links status every 500ms

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        status: (linkId: string) => get(store).get(linkId),
        statuses: () => get(store).values(),
        kill: () => clearInterval(interval)
    }
} ()
