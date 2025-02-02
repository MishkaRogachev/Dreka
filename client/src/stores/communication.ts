import { writable, get } from 'svelte/store';

import { type LinkDescription, type LinkStatus } from '$bindings/communication';

import type { WsListener } from "$datasource/ws";
import { ClientSideEvents, EventsService } from "$services/events";
import { CommunicationService } from '$services/communication';

export class Link {
    constructor(description: LinkDescription) {
        this.description = description;
    }
    description: LinkDescription
    status: LinkStatus | undefined
}

export const links = function () {
    let linkUpserted: WsListener;
    let linkRemoved: WsListener;
    let statusUpdated: WsListener;
    let wsConnected: WsListener;

    const store = writable(new Map<string, Link>(), (_, update) => {
        linkUpserted = (data: any) => {
            let link = data["link"] as LinkDescription;
            if (!link) {
                return;
            }

            update(links => {
                if (links.has(link.id)) {
                    links.get(link.id)!.description = link;
                } else {
                    links.set(link.id, new Link(link));
                }
                return links;
            });
        }

        linkRemoved = (data: any) => {
            let link_id = data["link_id"] as string;
            if (!link_id) {
                return;
            }

            update(links => {
                if (links.has(link_id)) {
                    links.delete(link_id);
                }
                return links;
            });
        }

        statusUpdated = (data: any) => {
            let status = data["status"] as LinkStatus;
            if (!status) {
                return;
            }

            update(links => {
                if (links.has(status.id)) {
                    links.get(status.id)!.status = status;
                }
                return links;
            });
        }

        wsConnected = async (_data: any) => {
            let descriptions = await CommunicationService.getLinkDescriptions();
            if (descriptions) {
                let links = new Map(descriptions!.map(description => [description.id, new Link(description)]));
                for (let [id, link] of links) {
                    let status = await CommunicationService.getLinkStatus(id)
                    if (status) {
                        link.status = status;
                    }
                }
                update(_ => { return links; });
            }
        }

        EventsService.subscribe("LinkUpserted", linkUpserted);
        EventsService.subscribe("LinkRemoved", linkRemoved);
        EventsService.subscribe("LinkStatusUpdated", statusUpdated);
        EventsService.subscribe(ClientSideEvents.WsConnectionOpened, wsConnected);
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
                    const id = descriptionBack!.id;
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
                    let removed = links.delete(linkIdBack!);
                    return links;
                })
            }
        },
        setLinkEnabled: async (linkId: string, connected: boolean) => {
            await CommunicationService.setLinkEnabled(linkId, connected);
        },
        kill: () => {
            EventsService.unsubscribe("LinkUpserted", linkUpserted);
            EventsService.unsubscribe("LinkRemoved", linkRemoved);
            EventsService.unsubscribe("LinkStatusUpdated", statusUpdated);
            EventsService.unsubscribe(ClientSideEvents.WsConnectionOpened, wsConnected);
        }
    }
} ()
