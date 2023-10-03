import { readable, writable, get, type Writable } from 'svelte/store';

import { type LinkDescription } from '$bindings/links';
import { LinksService } from '$services/links';

export const communicationLinks = readable(Array<LinkDescription>(), (set) => {
    const interval = setInterval(() => {
        LinksService.getLinks().then((vehicles: Array<LinkDescription>) => { set(vehicles); });
    }, 1000);

    return () => clearInterval(interval);
})