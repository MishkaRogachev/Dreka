use std::sync::Arc;
use surrealdb::{engine::local::Db, Surreal};

use crate::persistence::{repository, traits};
use crate::models::commands::{CommandId, CommandState, VehicleCommand, VehicleCommandState};
use crate::models::vehicles::VehicleId;

#[derive(Clone)]
pub struct Persistence {
    pub vehicle_commands: Arc<dyn traits::IRepository<VehicleCommandState> + Send + Sync>
}

impl Persistence {
    pub fn new(db: Surreal<Db>) -> Self {
        Self {
            vehicle_commands: Arc::new(repository::Repository::new(db.clone(), "vehicle_commands")),
        }
    }

    pub async fn register_vehicle_command(&self, vehicle_id: &VehicleId, command: &VehicleCommand) -> anyhow::Result<VehicleCommandState> {
        self.vehicle_commands.create(&VehicleCommandState {
            id: String::new(),
            vehicle_id: vehicle_id.clone(),
            command: command.clone(),
            attempt: 0,
            state: CommandState::Initial
        }).await
    }

    pub async fn get_vehicle_command(&self, command_id: &CommandId) -> anyhow::Result<VehicleCommandState> {
        self.vehicle_commands.read(command_id).await
    }

    pub async fn update_vehicle_command(&self, command: &VehicleCommandState) -> anyhow::Result<VehicleCommandState> {
        self.vehicle_commands.update(command).await
    }

    pub async fn drop_vehicle_command(&self, command_id: &CommandId) -> anyhow::Result<()> {
        self.vehicle_commands.delete(command_id).await?;
        Ok(())
    }

    pub async fn all_vehicle_commands(&self) -> anyhow::Result<Vec<VehicleCommandState>> {
        self.vehicle_commands.read_all().await
    }
}
