import { writable, get } from 'svelte/store';

import { type LinkDescription, type LinkStatus } from '$bindings/communication';
import { CommunicationService } from '$services/communication';

export const all_links = writable(new Map<String, LinkDescription>())

export async function getLinkStatus(id: string): Promise<LinkStatus> {
    return await CommunicationService.getLinkStatus(id) || {
        id: id,
        is_connected: false,
        is_online: false,
        bytes_received: 0.0,
        bytes_sent: 0.0
    };
}

export async function saveLink(link: LinkDescription): Promise<LinkDescription | null> {
    let linkBack = await CommunicationService.saveLink(link);
    if (linkBack && linkBack.id) {
        let links = get(all_links);
        links.set(linkBack.id, linkBack);
        all_links.set(links);
        return linkBack;
    }
    return null
}

export async function removeLink(linkId: string) {
    let linkIdBack = await CommunicationService.removeLink(linkId);
    let links = get(all_links);
    if (linkIdBack) {
        links.delete(linkIdBack);
    }
    all_links.set(links);
}

export async function setLinkConnected(linkId: string, connected: boolean) {
    await CommunicationService.setLinkConnected(linkId, connected);
}

// Refresh comm links every second
setInterval(() => {
    CommunicationService.getLinks().then((links: Array<LinkDescription>) => {
        let new_links = new Map<String, LinkDescription>();
        links.forEach((link: LinkDescription) => {
            if (link.id) {
                new_links.set(link.id, link);
            }
        })
        all_links.set(new_links);
    });
}, 1000);
