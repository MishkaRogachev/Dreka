use std::sync::Arc;
use surrealdb::{engine::local::Db, Surreal};

use crate::models::events::ServerEvent;

use super::{bus, commands, communication, vehicles, missions};

#[derive(Clone)]
pub struct Registry {
    pub communication: Arc<communication::Persistence>,
    pub vehicles: Arc<vehicles::Persistence>,
    pub commands: Arc<commands::Persistence>,
    pub missions: Arc<missions::Persistence>,
}

impl Registry {
    pub fn new(db: Surreal<Db>, bus: bus::EventBus<ServerEvent>) -> Self {
        let communication = Arc::new(communication::Persistence::new(db.clone(), bus.clone()));
        let vehicles = Arc::new(vehicles::Persistence::new(db.clone(), bus.clone()));
        let commands = Arc::new(commands::Persistence::new(db.clone(), bus.clone()));
        let missions = Arc::new(missions::Persistence::new(db, bus));

        Self { communication, vehicles, commands, missions }
    }
}