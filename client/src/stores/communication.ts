import { readable, writable, get } from 'svelte/store';

import { type LinkDescription, type LinkStatus } from '$bindings/communication';
import { CommunicationService } from '$services/communication';

export const linkDescriptions = function () {
    let interval: NodeJS.Timeout;

    const store = writable(new Map<string, LinkDescription>(), (_, update) => {
        interval = setInterval(async () => {
            let new_links = new Map<string, LinkDescription>();
            for (const link of await CommunicationService.getLinks()) {
                if (link.id) {
                    new_links.set(link.id, link);
                }
                update(links => {
                    new_links.forEach((link, id) => {
                        links.set(id, link);
                    });
                    return links;
                });
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
            if (linkBack && linkBack.id) {
                store.update(links => {
                    // @ts-ignore
                    links.set(linkBack.id, linkBack);
                    return links;
                });
            }
            return linkBack;
        },
        removeLink: async (linkId: string) => {
            let linkIdBack = await CommunicationService.removeLink(linkId);
            if (linkIdBack) {
                store.update(links => {
                    // @ts-ignore
                    links.delete(linkIdBack);
                    return links;
                });
            }
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
