import { writable, get } from 'svelte/store';

import { type LinkDescription, type LinkStatus } from '$bindings/communication';
import { CommunicationService } from '$services/communication';

export class Link {
    constructor(description: LinkDescription) {
        this.description = description;
    }
    description: LinkDescription
    status: LinkStatus | undefined
}

export const links = function () {
    let descriptionInterval: NodeJS.Timeout;
    let statusInterval: NodeJS.Timeout;

    const store = writable(new Map<string, Link>(), (_, update) => {
        // TODO: stop intervals if server is down
        descriptionInterval = setInterval(async () => {
            let descriptions = await CommunicationService.getLinkDescriptions();
            if (!descriptions) {
                return;
            }
            update(links => {
                let usedIds = new Array<string>();

                // Add and update existing links
                for (const description of descriptions!) {
                    const id = description.id!;
                    usedIds.push(id);

                    if (links.has(id)) {
                        links.get(id)!.description = description
                    } else {
                        links.set(id, new Link(description))
                    }
                }

                // Delete links removed by server
                for (const id of links.keys()) {
                    if (!usedIds.includes(id)) {
                        links.delete(id)
                    }
                }
                return links;
            });
        }, 2000); // Refresh description every second

        statusInterval = setInterval(async () => {
            let statuses = await CommunicationService.getLinkStatuses();
            if (!statuses) {
                return;
            }
            update(links => {
                for (const status of statuses!) {
                    if (links.has(status.id)) {
                        links.get(status.id)!.status = status
                    }
                }
                return links;
            });
        }, 500); // Refresh status every 500ms
    });

    return {
        subscribe: store.subscribe,
        count: () => get(store).size,
        link: (linkId: string) => get(store).get(linkId),
        linksIds: () => Array.from(get(store).keys()),
        links: () => get(store).values(),
        saveLink: async (description: LinkDescription) => {
            let descriptionBack = await CommunicationService.saveLinkDescription(description);
            let link: Link | undefined
            if (descriptionBack && descriptionBack.id) {
                store.update(links => {
                    const id = description.id!;
                    if (links.has(id)) {
                        links.get(id)!.description = descriptionBack!
                    } else {
                        links.set(id, new Link(descriptionBack!))
                    }
                    link = links.get(id)
                    return links;
                })
            }
            return link;
        },
        removeLink: async (linkId: string) => {
            let linkIdBack = await CommunicationService.removeLink(linkId);
            if (linkIdBack) {
                store.update(links => {
                    // @ts-ignore
                    links.delete(linkIdBack);
                    return links;
                })
            }
        },
        setLinkEnabled: async (linkId: string, connected: boolean) => {
            await CommunicationService.setLinkEnabled(linkId, connected);
        },
        kill: () => {
            clearInterval(descriptionInterval);
            clearInterval(statusInterval);
        }
    }
} ()
