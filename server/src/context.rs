use std::sync::Arc;
use tokio::sync::broadcast;
use surrealdb::{engine::local::Db, Surreal};

use crate::models::events;
use crate::persistence::{communication, vehicles};

#[derive(Clone)]
pub struct AppContext {
    pub communication: Arc<communication::Persistence>,
    pub vehicles: Arc<vehicles::Persistence>,
    pub tx: broadcast::Sender<events::ClentEvent>
}

impl AppContext {
    pub fn new(db: Surreal<Db>, tx: broadcast::Sender<events::ClentEvent>) -> Self {
        let communication = Arc::new(communication::Persistence::new(db.clone()));
        let vehicles = Arc::new(vehicles::Persistence::new(db));
        Self { communication, vehicles, tx }
    }
}
