use std::{sync::Arc, collections::HashMap};
use tokio::{time, sync::Mutex, sync::broadcast::Receiver};
use mavlink::{MavHeader, common::*};

use crate::models::commands::*;
use crate::models::events::ClientEvent;

use super::super::context::MavlinkContext;
use super::protocol;

const MAX_COMMAND_SEND_ATTEMPTS: u8 = 5;
const COMMAND_SEND_INTERVAL: time::Duration = time::Duration::from_millis(2000);

pub struct CommandHandler {
    context: Arc<Mutex<MavlinkContext>>,
    executions_last_sent: HashMap<CommandId, time::Instant>,
    waiting_ack_executions: HashMap<(u16, u8), CommandId>
}

impl CommandHandler {
    pub fn new(context: Arc<Mutex<MavlinkContext>>) -> Self {
        Self { context, executions_last_sent: HashMap::new(), waiting_ack_executions: HashMap::new() }
    }

    async fn add_command_execution(&mut self, request: ExecuteCommandRequest, command_id: CommandId) {
        let context = self.context.lock().await;

        let execution = CommandExecution {
            id: command_id.clone(),
            command: request.command.clone(),
            state: CommandState::Initial {},
            executor: request.executor.clone(),
        };

        if let Err(err) = context.registry.commands.save_execution(&execution).await {
            log::error!("Error saving command execution: {}", err);
        }
    }

    async fn finish_comand_execution(&mut self, mut execution: CommandExecution, state: CommandState) {
        let context = self.context.lock().await;

        // Mark as finished
        execution.state = state;
        if let Err(err) = context.registry.commands.update_execution(execution.clone()) {
            log::error!("Error updating command execution: {}", err);
        }

        // Remove next
        let keys_to_remove: Vec<(u16, u8)> = self.waiting_ack_executions
            .iter()
            .filter(|&(_, value)| value == &execution.id)
            .map(|(&key, _)| key)
            .collect();
        for key in keys_to_remove {
            self.waiting_ack_executions.remove(&key);
        }

        self.executions_last_sent.remove(&execution.id);
        if let Err(err) = context.registry.commands.remove_execution(&execution.id).await {
            log::error!("Error removing command execution: {}", err);
        }
    }

    async fn cancel_command_execution(&mut self, command_id: CommandId) {
        let execution; {
            let context = self.context.lock().await;
            match context.registry.commands.get_execution(&command_id).await {
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

        let context = self.context.lock().await;
        if let Err(err) = context.registry.commands.save_execution(&execution).await {
            log::error!("Error saving command execution: {}", err);
        }
    }

    async fn handle_client_event(&mut self, event: ClientEvent) {
        match event {
            ClientEvent::ExecuteCommand { request, command_id } => {
                self.add_command_execution(request, command_id).await;
            },
            ClientEvent::CancelCommand { command_id } => {
                self.cancel_command_execution(command_id).await;
            },
            _ => {}
        }
    }

    async fn process_execution(&mut self, execution: CommandExecution) -> Option<MavMessage> {
        // Early return if interval not exceeded, if even it's not in CommandState::Sent state
        if let Some(interval) = self.executions_last_sent.get(&execution.id) {
            if interval.elapsed() < COMMAND_SEND_INTERVAL {
                return None;
            }
        }

        // Get MAV ID for Vehicle
        let mav_id; {
            let context = self.context.lock().await;
            if let CommandExecutor::Vehicle { ref vehicle_id } = execution.executor {
                let mav_id_opt = context.mav_id_from_vehicle_id(&vehicle_id);
                if mav_id_opt.is_none() {
                    log::warn!("Vehicle not found: {}", vehicle_id);
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
            // Use -1 for protocol confirmation
            let encoded = protocol::encode_command(&execution.command, mav_id, attempt - 1);
            if let Some(encoded) = encoded {
                log::info!("Sending command: {:?}", execution);
                self.waiting_ack_executions.insert((encoded.cmd as u16, mav_id), execution.id.clone());
                self.executions_last_sent.insert(execution.id.clone(), time::Instant::now());
                self.save_command_execution(execution, state).await;
                return Some(encoded.message);
            } else {
                self.finish_comand_execution(execution, CommandState::Unsupported {}).await;
                return None;
            }
        }
        None
    }

    async fn collect_execution_messages(&mut self) -> Vec<MavMessage> {
        let mut messages = Vec::new();

        let executions: Vec<CommandExecution>; {
            let context = self.context.lock().await;
            match context.registry.commands.get_all_executions().await {
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

    async fn handle_ack(&mut self, mav_id: u8, ack: &COMMAND_ACK_DATA) {
        let id = self.waiting_ack_executions.get(&(ack.command as u16, mav_id));
        if id.is_none() {
            return;
        }

        let execution: CommandExecution; {
            let context = self.context.lock().await;
            match context.registry.commands.get_execution(&id.clone().unwrap()).await {
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

    pub async fn prepare_messages(&mut self, client_events_rx: &mut Receiver<ClientEvent>) -> Vec<MavMessage> {
        match client_events_rx.try_recv() {
            Ok(event) => self.handle_client_event(event).await,
            Err(err) => {
                if err != tokio::sync::broadcast::error::TryRecvError::Empty {
                    log::error!("RX error: {}", err);
                }
            }
        }
        self.collect_execution_messages().await
    }

    pub async fn handle_message(&mut self, header: &MavHeader, msg: &MavMessage) {
        match msg {
            MavMessage::COMMAND_ACK(ack) => {
                self.handle_ack(header.system_id, ack).await;
            },
            _ => {}
        }
    }
}
