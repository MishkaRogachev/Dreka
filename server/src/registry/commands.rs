use std::sync::Arc;
use surrealdb::{engine::local::Db, Surreal};

use crate::persistence::{repository, traits};
use crate::models::events::ServerEvent;
use crate::models::vehicles::VehicleId;
use crate::models::commands::{CommandId, CommandState, VehicleCommand, VehicleCommandState};

use super::bus;

#[derive(Clone)]
pub struct Persistence {
    vehicle_commands: Arc<dyn traits::IRepository<VehicleCommandState> + Send + Sync>,
    bus: bus::EventBus<ServerEvent>
}

impl Persistence {
    pub fn new(db: Surreal<Db>, bus: bus::EventBus<ServerEvent>) -> Self {
        Self {
            vehicle_commands: Arc::new(repository::Repository::new(db.clone(), "vehicle_commands")),
            bus
        }
    }

    pub async fn register_vehicle_command(&self, vehicle_id: &VehicleId, command: &VehicleCommand) -> anyhow::Result<VehicleCommandState> {
        let command_state = self.vehicle_commands.create(&VehicleCommandState {
            id: String::new(),
            vehicle_id: vehicle_id.clone(),
            command: command.clone(),
            attempt: 0,
            state: CommandState::Initial
        }).await?;

        self.bus.publish(ServerEvent::CommandUpdated { command: command_state.clone() })?;
        Ok(command_state)
    }

    pub async fn get_vehicle_command(&self, command_id: &CommandId) -> anyhow::Result<VehicleCommandState> {
        self.vehicle_commands.read(command_id).await
    }

    pub async fn update_vehicle_command(&self, command_state: &VehicleCommandState) -> anyhow::Result<VehicleCommandState> {
        let command_state = self.vehicle_commands.update(command_state).await?;

        self.bus.publish(ServerEvent::CommandUpdated { command: command_state.clone() })?;
        Ok(command_state)
    }

    pub async fn drop_vehicle_command(&self, command_id: &CommandId) -> anyhow::Result<()> {
        self.vehicle_commands.delete(command_id).await?;

        self.bus.publish(ServerEvent::CommandRemoved { command_id: command_id.clone() })?;
        Ok(())
    }

    pub async fn all_vehicle_commands(&self) -> anyhow::Result<Vec<VehicleCommandState>> {
        self.vehicle_commands.read_all().await
    }
}
