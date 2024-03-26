use std::sync::Arc;
use tokio::sync::Mutex;
use mavlink::{common::MavMessage, MavHeader};

use crate::models::commands::{CommandState, VehicleCommand, VehicleCommandState};

use super::super::context::MavlinkContext;
use super::protocol;

const MAX_COMMAND_SEND_ATTEMPTS: u8 = 5;

pub struct CommandHandler {
    context: Arc<Mutex<MavlinkContext>>
}

impl CommandHandler {
    pub fn new(context: Arc<Mutex<MavlinkContext>>) -> Self {
        Self { context }
    }

    // TODO: move to separate file
    fn check_command(&self, command: &VehicleCommandState) -> bool {
        if command.state == CommandState::Initial {
            return true;
        }
        // TODO: repeat commands
        // if command.state == CommandState::Sent && command.attempt < MAX_COMMAND_SEND_ATTEMPTS {
        //     return true;
        // }
        false
    }

    fn encode_command(&self, command: &VehicleCommandState, mav_id: u8) -> Option<mavlink::common::MavMessage> {
        match &command.command {
            VehicleCommand::ArmDisarm { arm } => Option::Some(protocol::arm_disarm(mav_id, *arm, command.attempt)),
            VehicleCommand::SetWaypoint { wp } => Option::Some(protocol::set_waypoint(mav_id, *wp)),
            VehicleCommand::NavTo { position } => Option::Some(protocol::nav_to(mav_id, position)),
            VehicleCommand::Takeoff { altitude } => Option::Some(protocol::takeoff(mav_id, *altitude, command.attempt)),
            VehicleCommand::GoAround {} => Some(protocol::go_around(mav_id, command.attempt)),
            VehicleCommand::SetServo { channel, value } => Some(protocol::set_servo(mav_id, *channel, *value, command.attempt)),
            VehicleCommand::OverrideServos { servos } => Some(protocol::override_servos(mav_id, &servos)),
            _ => None
        }
    }

    pub async fn process_command(&self, context: &tokio::sync::MutexGuard<'_, MavlinkContext>,
                                mut command: VehicleCommandState, mav_id: u8) -> Option<mavlink::common::MavMessage> {
        if !self.check_command(&command) {
            return None;
        }

        if let Some(msg) = self.encode_command(&command, mav_id) {
            command.state = CommandState::Sent;
            command.attempt += 1;
            if let Err(err) = context.registry.commands.update_vehicle_command(&command).await {
                log::error!("Error updating vehicle command: {:?}", err);
            }
            // TODO: mark command as sent and increate attempts
            return Some(msg);
        }
        None
    }

    pub async fn process_commands(&mut self) -> Vec<mavlink::common::MavMessage> {
        let context = self.context.lock().await;
        let mut result = Vec::new();

        // TODO: should be relaced with a command queue
        match context.registry.commands.all_vehicle_commands().await {
            Ok(commands) => {
                for command in commands {
                    match context.mav_id_from_vehicle_id(&command.vehicle_id){
                        Some(mav_id) => {
                            if let Some(msg) = self.process_command(&context, command, mav_id).await {
                                result.push(msg);
                            }
                        },
                        None => {
                            log::warn!("Vehicle not found for command: {:?}", command);
                            continue;
                        }
                    }
                    // TODO: cancel commands
                }
            },
            Err(err) => {
                log::error!("Error getting all vehicle commands: {}", err);
            },
        }
        result
    }

    pub async fn handle_message(&mut self, header: &MavHeader, msg: &MavMessage) {
        // TODO: handle acts
    }
}