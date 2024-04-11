use crate::db::surreal_dao::Dao;
use crate::bus::bus::EventBus;

use crate::models::events::ServerEvent;

#[derive(Clone)]
pub struct Dal {
    pub dao: Dao,
    pub bus: EventBus<ServerEvent>
}

impl Dal {
    pub fn new(dao: Dao, bus: EventBus<ServerEvent>) -> Self {
        Self { dao, bus }
    }
}
