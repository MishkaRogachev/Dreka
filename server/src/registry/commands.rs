use std::sync::Arc;
use anyhow::Ok;
use surrealdb::{engine::local::Db, Surreal};

use crate::persistence::{repository, traits};
use crate::models::events::ServerEvent;
use crate::models::commands::{CommandExecution, CommandId, CommandExecutor};

use super::bus;

#[derive(Clone)]
pub struct Persistence {
    executions: Arc<dyn traits::IRepository<CommandExecution> + Send + Sync>,
    bus: bus::EventBus<ServerEvent>
}

impl Persistence {
    pub fn new(db: Surreal<Db>, bus: bus::EventBus<ServerEvent>) -> Self {
        Self {
            executions: Arc::new(repository::Repository::new(db.clone(), "vehicle_commands")),
            bus
        }
    }

    pub fn update_execution(&self, execution: CommandExecution) -> anyhow::Result<()> {
        self.bus.publish(ServerEvent::CommandExecutionUpdated { execution })
    }

    pub async fn save_execution(&self, execution: &CommandExecution) -> anyhow::Result<CommandExecution> {
        let execution = if execution.id.is_empty() {
            self.executions.create(execution).await?
        } else {
            self.executions.update(execution).await?
        };

        self.update_execution(execution.clone())?;
        Ok(execution)
    }

    pub async fn remove_execution(&self, id: &CommandId) -> anyhow::Result<()> {
        self.executions.delete(id).await?;

        self.bus.publish(ServerEvent::CommandExecutionRemoved { command_id: id.into() })?;
        Ok(())
    }

    pub async fn get_execution(&self, id: &CommandId) -> anyhow::Result<CommandExecution> {
        self.executions.read(id).await
    }

    pub async fn get_all_executions(&self) -> anyhow::Result<Vec<CommandExecution>> {
        self.executions.read_all().await
    }
}
