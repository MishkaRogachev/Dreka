const WS_URL = "ws://127.0.0.1:45486/telemetry/ws"

export function ws_telemetry() {
    return new WebSocket("ws://127.0.0.1:45486/telemetry/ws");
}