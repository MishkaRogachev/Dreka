import { readable, derived } from 'svelte/store';

import { type LinkDescription, type LinkStatus } from '$bindings/communication';
import { CommunicationService } from '$services/communication';

export const links = readable(Array<LinkDescription>(), (set) => {
    const interval = setInterval(() => {
        CommunicationService.getLinks().then((links: Array<LinkDescription>) => { set(links); });
    }, 1000);

    return () => clearInterval(interval);
})

export const linkStatuses = derived(links,
    ($links) => {
        let statuses = new Map()
        $links.forEach(link => {
            CommunicationService.getLinkStatus(link.id!).then(status => {
                if (!!status) {
                    statuses.set(status?.id.id.String, status);
                }
            });
        });
        return statuses;
    }
);