import { readable } from 'svelte/store';

import { type LinkDescription, type LinkStatus } from '$bindings/communication';
import { CommunicationService } from '$services/communication';

export const links = readable(Array<LinkDescription>(), (set) => {
    const interval = setInterval(() => {
        CommunicationService.getLinks().then((links: Array<LinkDescription>) => { set(links); });
    }, 1000);

    return () => clearInterval(interval);
})

export async function getLinkStatus(id: string): Promise<LinkStatus> {
    return await CommunicationService.getLinkStatus(id) || {
        id: id,
        is_connected: false,
        is_online: false
    };
}

export async function saveLink(link: LinkDescription) {
    return await CommunicationService.saveLink(link);
}
