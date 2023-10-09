import { readable, derived } from 'svelte/store';

import { type LinkDescription, type LinkStatus } from '$bindings/communication';
import { CommunicationService } from '$services/communication';

export const links = readable(Array<LinkDescription>(), (set) => {
    const interval = setInterval(() => {
        CommunicationService.getLinks().then((links: Array<LinkDescription>) => { set(links); });
    }, 1000);

    return () => clearInterval(interval);
})

export let linkStatuses: Map<String, LinkStatus> = new Map()

links.subscribe((links: Array<LinkDescription>) => {
    links.forEach(link => {
        CommunicationService.getLinkStatus(link.id!).then(status => {
            if (!!status) {
                linkStatuses.set(status?.id, status);
            }
        });
    });
})
