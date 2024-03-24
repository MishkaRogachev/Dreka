import type { VehicleTelemetry } from "$bindings/telemetry";
import { WS_TELEMETRY_URL, WsWatchdog } from "$datasource/ws";

export class TelemetryService {
    static start() {
        this.watchdog.start();
    }

    static stop() {
        this.watchdog.stop();
    }

    static subscribeToTelemetry(cb: (telemetry: VehicleTelemetry) => void) {
        this.watchdog.subscribe((data: any) => {
            cb(JSON.parse(data));
        });
    }

    private static watchdog = new WsWatchdog(WS_TELEMETRY_URL);
}
