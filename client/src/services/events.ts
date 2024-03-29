import type { ServerEvent } from "$bindings/events";
import { WEBSOCKET_URL, WsWatchdog, type WsListener } from "$datasource/ws";

export class EventsContext {
    watchdog = new WsWatchdog(WEBSOCKET_URL);
    listeners: Map<keyof ServerEvent, WsListener[]> = new Map();
}

export class EventsService {
    static init() {
        this.context.watchdog.subscribe((data: any) => {
            let event = JSON.parse(data) as ServerEvent;
            if (!event) {
                console.warn("Invalid event received: ", data);
                return;
            }
            let eventType = Object.keys(event)[0] as keyof ServerEvent;

            let eventListeners = this.context.listeners.get(eventType);
            if (eventListeners) {
                for (const listener of eventListeners) {
                    listener(event[eventType]);
                }
            }
        });
        this.context.watchdog.start();
    }

    static done() {
        this.context.watchdog.stop();
        this.context.listeners.clear();
    }

    static subscribe(eventType: keyof ServerEvent, listener: WsListener) {
        const eventListeners = this.context.listeners.get(eventType) || [];
        eventListeners.push(listener);
        this.context.listeners.set(eventType, eventListeners);
    }

    static unsubscribe(eventType: keyof ServerEvent, listener: WsListener) {
        const eventListeners = this.context.listeners.get(eventType);
        if (eventListeners) {
            const index = eventListeners.indexOf(listener);
            if (index !== -1) {
                eventListeners.splice(index, 1);
            }
        }
    }

    private static context = new EventsContext();
}