import type { ServerEvent } from "$bindings/events";
import { WEBSOCKET_URL, WsWatchdog, type WsListener } from "$datasource/ws";

export class EventsContext {
    watchdog = new WsWatchdog(WEBSOCKET_URL);
    listeners: Map<keyof ServerEvent, WsListener> = new Map();
}

export class EventsService {
    static init() {
        this.context.watchdog.subscribe((data: any) => {
            let event = JSON.parse(data) as ServerEvent;
            if (!event) {
                console.warn("Invalid event received: ", data);
                return;
            }

            this.context.listeners.forEach((listener, eventType) => {
                listener(event[eventType]);
            });
        });
        this.context.watchdog.start();
    }

    static done() {
        this.context.watchdog.stop();
        this.context.listeners.clear();
    }

    static subscribe(eventType: keyof ServerEvent, listener: WsListener) {
        this.context.listeners.set(eventType, listener);
    }

    static unsubscribe(eventType: keyof ServerEvent) {
        this.context.listeners.delete(eventType);
    }

    private static context = new EventsContext();
}