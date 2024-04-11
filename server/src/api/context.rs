use crate::models::events::{ClientEvent, ServerEvent};
use crate::{bus::bus, dal::dal};

#[derive(Clone)]
pub struct ApiContext {
    pub dal: dal::Dal,
    pub server_bus: bus::EventBus::<ServerEvent>,
    pub client_bus: bus::EventBus::<ClientEvent>,
}

impl ApiContext {
    pub fn new(
        dal: dal::Dal,
        server_bus: bus::EventBus::<ServerEvent>,
        client_bus: bus::EventBus::<ClientEvent>,
    ) -> Self {
        Self { dal, server_bus, client_bus }
    }
}
