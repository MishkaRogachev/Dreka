use crate::models::events::{ClientEvent, ServerEvent};
use crate::registry::{bus, registry};

#[derive(Clone)]
pub struct ApiContext {
    pub registry: registry::Registry,
    pub server_bus: bus::EventBus::<ServerEvent>,
    pub client_bus: bus::EventBus::<ClientEvent>,
}

impl ApiContext {
    pub fn new(
        registry: registry::Registry,
        server_bus: bus::EventBus::<ServerEvent>,
        client_bus: bus::EventBus::<ClientEvent>,
    ) -> Self {
        Self { registry, server_bus, client_bus }
    }
}
