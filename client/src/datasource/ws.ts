export const WEBSOCKET_URL = "ws://127.0.0.1:45486/ws"

export type WsListener = (data: any) => void;

export class WsWatchdog {
    constructor(url: string) {
        this.ws = null;
        this.listeners = [];
        this.interval = null;
        this.url = url;
    }

    start() {
        if (this.interval) {
            this.stop();
        }

        this.interval = setInterval(() => {
            if (!this.ws) {
                this.connect();
            }
        }, 1000);
    }

    stop() {
        if (this.ws) {
            this.ws.close();
        }
        if (this.interval) {
            clearInterval(this.interval);
            this.interval = null;
        }
    }

    connect() {
        console.log("Connecting to WebSocket");
        this.ws = new WebSocket(this.url);

        this.ws.addEventListener("open", event => {
            console.log("WebSocket connection established");
        });

        this.ws.addEventListener("message", event => {
            this.listeners.forEach(listener => { listener(event.data); });
        });

        this.ws.addEventListener("close", event => { this.ws = null; });
        this.ws.addEventListener("error", event => { this.ws = null; });
    }

    subscribe(listener: WsListener) {
        this.listeners.push(listener);
    }

    unsubscribe(listener: WsListener) {
        this.listeners = this.listeners.filter(item => item !== listener);
    }

    private url: string;
    private interval: NodeJS.Timeout | null;
    private ws: WebSocket | null;
    private listeners: Array<Function>
}
