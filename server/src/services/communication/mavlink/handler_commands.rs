use tokio::time;
use mavlink::common::*;

use crate::models::commands::*;
use super::{handler, protocol::commands as protocol};

const MAX_COMMAND_SEND_ATTEMPTS: u8 = 5;
const COMMAND_RESEND_INTERVAL: time::Duration = time::Duration::from_millis(2000);

impl handler::Handler {
    pub async fn add_command_execution(&mut self, request: ExecuteCommandRequest, command_id: CommandId) {
        let execution = CommandExecution {
            id: command_id.clone(),
            command: request.command.clone(),
            state: CommandState::Initial {},
            executor: request.executor.clone(),
        };

        if let Err(err) = self.dal.save_command_execution(execution).await {
            log::error!("Error saving command execution: {}", err);
        }
    }

    pub async fn finish_comand_execution(&mut self, mut execution: CommandExecution, state: CommandState) {
        // Mark as finished
        execution.state = state;
        if let Err(err) = self.dal.update_command_execution(execution.clone()) {
            log::error!("Error updating command execution: {}", err);
        }

        // Remove next
        let keys_to_remove: Vec<(u16, u8)> = self.waiting_ack_command_executions
            .iter()
            .filter(|&(_, value)| value == &execution.id)
            .map(|(&key, _)| key)
            .collect();
        for key in keys_to_remove {
            self.waiting_ack_command_executions.remove(&key);
        }

        self.command_executions_last_sent.remove(&execution.id);
        if let Err(err) = self.dal.remove_command_execution(&execution.id).await {
            log::error!("Error removing command execution: {}", err);
        }
    }

    pub async fn cancel_command_execution(&mut self, command_id: CommandId) {
        let execution; {
            match self.dal.command_execution(&command_id).await {
                Ok(exec) => execution = exec,
                Err(err) => {
                    log::error!("Error getting command execution: {}", err);
                    return;
                }
            }
        }

        // TODO: use https://mavlink.io/en/messages/common.html#COMMAND_CANCEL for long running commands
        log::info!("Canceling command: {:?}", execution);

        self.finish_comand_execution(execution, CommandState::Canceled {}).await
    }

    async fn save_command_execution(&mut self, mut execution: CommandExecution, state: CommandState) {
        execution.state = state;

        if let Err(err) = self.dal.save_command_execution(execution).await {
            log::error!("Error saving command execution: {}", err);
        }
    }

    async fn process_execution(&mut self, execution: CommandExecution) -> Option<MavMessage> {
        // Early return if interval not exceeded, if even it's not in CommandState::Sent state
        if let Some(interval) = self.command_executions_last_sent.get(&execution.id) {
            if interval.elapsed() < COMMAND_RESEND_INTERVAL {
                return None;
            }
        }

        // Get MAV ID for Vehicle
        let mav_id; {
            if let CommandExecutor::Vehicle { ref vehicle_id } = execution.executor {
                let mav_id_opt = self.mav_id_from_vehicle_id(&vehicle_id);
                if mav_id_opt.is_none() {
                    log::warn!("Vehicle not found: {}", vehicle_id);
                    self.finish_comand_execution(execution, CommandState::Failed {}).await;
                    return None;
                }
                mav_id = mav_id_opt.unwrap();
            } else {
                // Skip if executor is not a vehicle
                return None;
            }
        }

        // Check if exeeded max attempts
        let state;
        match execution.state {
            CommandState::Initial {} => {
                state = CommandState::Sent { attempt: 1 };
            },
            CommandState::Sent { attempt } => {
                if attempt < MAX_COMMAND_SEND_ATTEMPTS {
                    state = CommandState::Sent { attempt: attempt + 1 };
                } else {
                    self.finish_comand_execution(execution, CommandState::Failed {}).await;
                    return None;
                }
            },
            _ => return None
        }

        // Try to encode command
        if let CommandState::Sent { attempt } = state {
            let encoded: Option<protocol::EncodedCommand>;

            // Special case for SetMode
            if let Command::SetMode { mode } = &execution.command {
                let modes = self.mav_modes.get(&mav_id);
                if modes.is_none() {
                    log::warn!("Modes are not initialised for vehicle: {}", mav_id);
                    return None;
                }
                let mode_code = modes.unwrap().iter()
                    .find(|(_, value)| mode == *value)
                    .map(|(&key, _)| key);
                if mode_code.is_none() {
                    log::warn!("Mode {:?} is not available for vehicle", mode);
                    return None;
                }
                encoded = Some(protocol::encode_set_mode(mode_code.unwrap(), mav_id, attempt - 1));
            } else {
                encoded = protocol::encode_command(&execution.command, mav_id, attempt - 1);
            }

            if let Some(encoded) = encoded {
                log::info!("Sending command: {:?}", execution);
                if let Some(ack_cmd) = encoded.ack_cmd {
                    self.waiting_ack_command_executions.insert((ack_cmd as u16, mav_id), execution.id.clone());
                }
                self.command_executions_last_sent.insert(execution.id.clone(), time::Instant::now());
                self.save_command_execution(execution, state).await;
                return Some(encoded.message);
            } else {
                self.finish_comand_execution(execution, CommandState::Unsupported {}).await;
                return None;
            }
        }
        None
    }

    pub async fn collect_command_messages(&mut self) -> Vec<MavMessage> {
        let mut messages = Vec::new();

        let executions: Vec<CommandExecution>; {
            match self.dal.all_command_executions().await {
                Ok(execs) => executions = execs,
                Err(err) => {
                    log::error!("Error getting executions: {}", err);
                    return messages;
                }
            }
        }

        for execution in executions {
            if let Some(message) = self.process_execution(execution).await {
                messages.push(message);
            }
        }
        messages
    }

    pub async fn handle_command_ack(&mut self, mav_id: u8, ack: &COMMAND_ACK_DATA) {
        let id = self.waiting_ack_command_executions.get(&(ack.command as u16, mav_id));
        if id.is_none() {
            return;
        }

        let execution: CommandExecution; {
            match self.dal.command_execution(&id.clone().unwrap()).await {
                Ok(exec) => execution = exec,
                Err(err) => {
                    log::error!("Can't command execution for ack: {}", err);
                    return;
                }
            }
        }

        match ack.result {
            MavResult::MAV_RESULT_ACCEPTED => {
                self.finish_comand_execution(execution, CommandState::Accepted {}).await
            },
            MavResult::MAV_RESULT_TEMPORARILY_REJECTED => {
                self.finish_comand_execution(execution, CommandState::Rejected {}).await
            },
            MavResult::MAV_RESULT_DENIED => {
                self.finish_comand_execution(execution, CommandState::Denied {}).await
            },
            MavResult::MAV_RESULT_UNSUPPORTED => {
                self.finish_comand_execution(execution, CommandState::Unsupported {}).await
            },
            MavResult::MAV_RESULT_FAILED => {
                self.finish_comand_execution(execution, CommandState::Failed {}).await
            },
            MavResult::MAV_RESULT_IN_PROGRESS => {
                // TODO: mavlink 2 progress
                self.save_command_execution(execution, CommandState::InProgress { progress: 0 }).await
            },
            MavResult::MAV_RESULT_CANCELLED => {
                self.finish_comand_execution(execution, CommandState::Canceled {}).await
            },
        }
    }
}
