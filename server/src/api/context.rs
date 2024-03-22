use crate::models::{events::ClentEvent, telemetry::VehicleTelemetry};
use crate::registry::registry;

#[derive(Clone)]
pub struct ApiContext {
    pub registry: registry::Registry,
    pub client_events_tx: flume::Sender<ClentEvent>,
    pub telemetry_rx: flume::Receiver<VehicleTelemetry>
}

impl ApiContext {
    pub fn new(
        registry: registry::Registry,
        client_events_tx: flume::Sender<ClentEvent>,
        telemetry_rx: flume::Receiver<VehicleTelemetry>
    ) -> Self {
        Self { registry, client_events_tx, telemetry_rx }
    }
}
