
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::{datasource::db::Repository, models};

#[derive(Clone)]
pub struct Shared {
    // TODO: separate temporary and persistence databases
    pub repository: Arc<Repository>,
    pub tx: broadcast::Sender<models::events::ClentEvent>
}

impl Shared {
    pub fn new(repository: Arc<Repository>, tx: broadcast::Sender<models::events::ClentEvent>) -> Self {
        Self { repository, tx }
    }
}
