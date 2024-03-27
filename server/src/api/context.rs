use crate::models::events::{ClentEvent, ServerEvent};
use crate::registry::registry;

#[derive(Clone)]
pub struct ApiContext {
    pub registry: registry::Registry,
    pub client_events: flume::Sender<ClentEvent>,
    pub server_events: flume::Receiver<ServerEvent>
}

impl ApiContext {
    pub fn new(
        registry: registry::Registry,
        client_events: flume::Sender<ClentEvent>,
        server_events: flume::Receiver<ServerEvent>
    ) -> Self {
        Self { registry, client_events, server_events }
    }
}
