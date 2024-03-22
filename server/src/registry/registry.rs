use std::sync::Arc;
use surrealdb::{engine::local::Db, Surreal};

use super::{communication, vehicles};

#[derive(Clone)]
pub struct Registry {
    pub communication: Arc<communication::Persistence>,
    pub vehicles: Arc<vehicles::Persistence>,
}

impl Registry {
    pub fn new(db: Surreal<Db>) -> Self {
        let communication = Arc::new(communication::Persistence::new(db.clone()));
        let vehicles = Arc::new(vehicles::Persistence::new(db));
        Self { communication, vehicles }
    }
}