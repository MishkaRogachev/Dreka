import type { ServerEvent } from "$bindings/events";
import { WEBSOCKET_URL, WsWatchdog, type WsListener, MsEventType } from "$datasource/ws";

export enum ClientSideEvents {
    WsConnectionOpened = "WsConnectionOpened",
    WsConnectionClosed = "WsConnectionClosed",
}

export class EventsContext {
    watchdog = new WsWatchdog(WEBSOCKET_URL);
    eventListeners: Map<string, WsListener[]> = new Map();
}

export class EventsService {
    static init() {
        this.context.watchdog.setCallback(MsEventType.Open, (event: any) => {
            console.log("WebSocket connection established");
            let eventListeners = this.context.eventListeners.get(ClientSideEvents.WsConnectionOpened);
            if (eventListeners) {
                for (const listener of eventListeners) {
                    listener(event);
                }
            }
        });

        this.context.watchdog.setCallback(MsEventType.Close, (event: any) => {
            console.log("Closing the websocket connection");
            let eventListeners = this.context.eventListeners.get(ClientSideEvents.WsConnectionClosed);
            if (eventListeners) {
                for (const listener of eventListeners) {
                    listener(event);
                }
            }
        });

        this.context.watchdog.setCallback(MsEventType.Message, (event: any) => {
            let message = JSON.parse(event.data) as ServerEvent;
            if (!message) {
                console.warn("Invalid message event received: ", message);
                return;
            }
            let eventType = Object.keys(message)[0] as keyof ServerEvent;

            let eventListeners = this.context.eventListeners.get(eventType);
            if (eventListeners) {
                for (const listener of eventListeners) {
                    listener(message[eventType]);
                }
            }
        });
        this.context.watchdog.start();
    }

    static done() {
        this.context.watchdog.stop();
        this.context.eventListeners.clear();
    }

    static subscribe(eventType: string, listener: WsListener) {
        const eventListeners = this.context.eventListeners.get(eventType) || [];
        eventListeners.push(listener);
        this.context.eventListeners.set(eventType, eventListeners);

        if (eventType === ClientSideEvents.WsConnectionOpened && this.context.watchdog.isConnected()) {
            listener(null);
        }
    }

    static unsubscribe(eventType: string, listener: WsListener) {
        const eventListeners = this.context.eventListeners.get(eventType);
        if (eventListeners) {
            const index = eventListeners.indexOf(listener);
            if (index !== -1) {
                eventListeners.splice(index, 1);
            }
        }
    }

    private static context = new EventsContext();
}
