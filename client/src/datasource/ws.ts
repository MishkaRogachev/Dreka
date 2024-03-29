export const WEBSOCKET_URL = "ws://127.0.0.1:45486/ws"

export type WsListener = (data: any) => void;

export enum MsEventType {
    Open = "open",
    Close = "close",
    Message = "message",
    Error = "error",
}

export class WsWatchdog {
    constructor(url: string) {
        this.ws = null;
        this.listeners = new Map<MsEventType, Function>();
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

        this.setupEventListener(MsEventType.Open);
        this.setupEventListener(MsEventType.Close);
        this.setupEventListener(MsEventType.Message);
        this.setupEventListener(MsEventType.Error);
    }

    private setupEventListener(eventType: MsEventType) {
        this.ws!.addEventListener(eventType, event => {
            const cb = this.listeners.get(eventType);
            if (cb) {
                cb(event);
            }
        });
    }

    setCallback(event: MsEventType, cb: WsListener) {
        this.listeners.set(event, cb);
    }

    private url: string;
    private interval: NodeJS.Timeout | null;
    private ws: WebSocket | null;
    private listeners: Map<MsEventType, Function>
}
