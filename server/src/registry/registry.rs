use std::sync::Arc;
use surrealdb::{engine::local::Db, Surreal};

use super::{communication, vehicles, commands};

#[derive(Clone)]
pub struct Registry {
    pub communication: Arc<communication::Persistence>,
    pub vehicles: Arc<vehicles::Persistence>,
    pub commands: Arc<commands::Persistence>,
}

impl Registry {
    pub fn new(db: Surreal<Db>) -> Self {
        let communication = Arc::new(communication::Persistence::new(db.clone()));
        let vehicles = Arc::new(vehicles::Persistence::new(db.clone()));
        let commands = Arc::new(commands::Persistence::new(db));
        Self { communication, vehicles, commands }
    }
}