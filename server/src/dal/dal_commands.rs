use super::dal::Dal;

use crate::models::events::ServerEvent;
use crate::models::commands::{CommandExecution, CommandId};

const TB_COMMANDS_EXECUTIONS: &str = "command_executions";

impl Dal {
    pub fn update_command_execution(&self, execution: CommandExecution) -> anyhow::Result<()> {
        self.bus.publish(ServerEvent::CommandExecutionUpserted { execution })
    }

    pub async fn save_command_execution(&self, execution: CommandExecution) -> anyhow::Result<CommandExecution> {
        let execution = if execution.id.is_empty() {
            self.dao.create(TB_COMMANDS_EXECUTIONS, execution).await?
        } else {
            self.dao.update(TB_COMMANDS_EXECUTIONS, execution).await?
        };
        self.update_command_execution(execution.clone())?;
        Ok(execution)
    }

    pub async fn remove_command_execution(&self, id: &CommandId) -> anyhow::Result<()> {
        self.dao.delete(TB_COMMANDS_EXECUTIONS, id).await?;
        self.bus.publish(ServerEvent::CommandExecutionRemoved { command_id: id.into() })?;
        Ok(())
    }

    pub async fn get_command_execution(&self, id: &CommandId) -> anyhow::Result<CommandExecution> {
        self.dao.select_one(TB_COMMANDS_EXECUTIONS, id).await
    }

    pub async fn get_all_command_executions(&self) -> anyhow::Result<Vec<CommandExecution>> {
        self.dao.select_all(TB_COMMANDS_EXECUTIONS).await
    }
}
