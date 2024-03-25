use std::sync::Arc;
use mavlink::{common::{MavCmd, MavMessage, COMMAND_LONG_DATA, MISSION_ITEM_DATA, MISSION_SET_CURRENT_DATA, RC_CHANNELS_OVERRIDE_DATA}, MavHeader};
use tokio::sync::Mutex;

use crate::models::{commands::{CommandState, VehicleCommand, VehicleCommandState}, spatial::Geodetic};

use super::context::MavlinkContext;

const MAX_COMMAND_SEND_ATTEMPTS: u8 = 5;

pub struct CommandHandler {
    context: Arc<Mutex<MavlinkContext>>
}

impl CommandHandler {
    pub fn new(context: Arc<Mutex<MavlinkContext>>) -> Self {
        Self { context }
    }

    fn arm_disarm(&self, mav_id: u8, arm: bool, attempt: u8) -> MavMessage {
        log::info!("Mav: {} Arm/Disarm: {}", mav_id, arm);
        MavMessage::COMMAND_LONG(COMMAND_LONG_DATA{
            param1: arm as i32 as f32,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            param5: 0.0,
            param6: 0.0,
            param7: 0.0,
            command: MavCmd::MAV_CMD_COMPONENT_ARM_DISARM,
            target_system: mav_id,
            target_component: mavlink::common::MavComponent::MAV_COMP_ID_ALL as u8,
            confirmation: attempt,
        })
    }

    fn set_waypoint(&self, mav_id: u8, wp: u16) -> MavMessage {
        log::info!("Mav: {} Set Waypoint: {}", mav_id, wp);
        MavMessage::MISSION_SET_CURRENT(MISSION_SET_CURRENT_DATA{
            seq: wp,
            target_system: mav_id,
            target_component: 0,
        })
    }

    fn nav_to(&self, mav_id: u8, position: &Geodetic) -> MavMessage {
        log::info!("Mav: {} Nav to: {:?}", mav_id, position);
        MavMessage::MISSION_ITEM(MISSION_ITEM_DATA{
            param1: 0.0,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            frame: mavlink::common::MavFrame::MAV_FRAME_GLOBAL_INT,
            command: mavlink::common::MavCmd::MAV_CMD_NAV_WAYPOINT,
            current: 2, // guided
            seq: 0,
            autocontinue: 0,
            target_system: mav_id,
            target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
            x: position.latitude as f32,
            y: position.longitude as f32,
            z: position.altitude
        })
    }

    fn takeoff(&self, mav_id: u8, altitude: f32, attempt: u8) -> MavMessage {
        log::info!("Mav: {} Takeoff to: {}", mav_id, altitude);
        MavMessage::COMMAND_LONG(COMMAND_LONG_DATA{
            param1: 0.0,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            param5: 0.0,
            param6: 0.0,
            param7: altitude,
            command: MavCmd::MAV_CMD_NAV_TAKEOFF,
            target_system: mav_id,
            target_component: mavlink::common::MavComponent::MAV_COMP_ID_ALL as u8,
            confirmation: attempt,
        })
    }

    fn go_around(&self, mav_id: u8, attempt: u8) -> MavMessage {
        log::info!("Mav: {} Go Around", mav_id);
        MavMessage::COMMAND_LONG(COMMAND_LONG_DATA{
            param1: 0.0,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            param5: 0.0,
            param6: 0.0,
            param7: 0.0,
            command: MavCmd::MAV_CMD_DO_GO_AROUND,
            target_system: mav_id,
            target_component: mavlink::common::MavComponent::MAV_COMP_ID_ALL as u8,
            confirmation: attempt,
        })
    }

    fn set_servo(&self, mav_id: u8, channel: u16, value: u16, attempt: u8) -> MavMessage {
        log::info!("Mav: {} Set Servo: {} to {}", mav_id, channel, value);
        MavMessage::COMMAND_LONG(COMMAND_LONG_DATA{
            param1: channel as f32,
            param2: value as f32,
            param3: 0.0,
            param4: 0.0,
            param5: 0.0,
            param6: 0.0,
            param7: 0.0,
            command: MavCmd::MAV_CMD_DO_SET_SERVO,
            target_system: mav_id,
            target_component: mavlink::common::MavComponent::MAV_COMP_ID_ALL as u8,
            confirmation: attempt,
        })
    }

    fn override_servos(&self, mav_id: u8, servos: &std::collections::BTreeMap<u16, u16>) -> MavMessage {
        log::info!("Mav: {} Override Servos: {:?}", mav_id, servos);
        MavMessage::RC_CHANNELS_OVERRIDE(RC_CHANNELS_OVERRIDE_DATA{
            target_system: mav_id,
            target_component: mavlink::common::MavComponent::MAV_COMP_ID_ALL as u8,
            chan1_raw: *servos.get(&0).unwrap_or(&0),
            chan2_raw: *servos.get(&1).unwrap_or(&0),
            chan3_raw: *servos.get(&2).unwrap_or(&0),
            chan4_raw: *servos.get(&3).unwrap_or(&0),
            chan5_raw: *servos.get(&4).unwrap_or(&0),
            chan6_raw: *servos.get(&5).unwrap_or(&0),
            chan7_raw: *servos.get(&6).unwrap_or(&0),
            chan8_raw: *servos.get(&7).unwrap_or(&0),
        })
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

    fn encode_command(&self, command: VehicleCommandState, mav_id: u8) -> Option<mavlink::common::MavMessage> {
        match command.command {
            VehicleCommand::ArmDisarm { arm } => Option::Some(self.arm_disarm(mav_id, arm, command.attempt)),
            VehicleCommand::SetWaypoint { wp } => Option::Some(self.set_waypoint(mav_id, wp)),
            VehicleCommand::NavTo { position } => Option::Some(self.nav_to(mav_id, &position)),
            VehicleCommand::Takeoff { altitude } => Option::Some(self.takeoff(mav_id, altitude, command.attempt)),
            VehicleCommand::GoAround {} => Some(self.go_around(mav_id, command.attempt)),
            VehicleCommand::SetServo { channel, value } => Some(self.set_servo(mav_id, channel, value, command.attempt)),
            VehicleCommand::OverrideServos { servos } => Some(self.override_servos(mav_id, &servos)),
            _ => None
        }
    }

    pub fn process_command(&self, command: VehicleCommandState, mav_id: u8) -> Option<mavlink::common::MavMessage> {
        if !self.check_command(&command) {
            return None;
        }

        if let Some(msg) = self.encode_command(command, mav_id) {
            // TODO: mark command as sent and increate attempts
            return Some(msg);
        }
        // TODO: cancel commands
        None
    }

    pub async fn process_commands(&mut self) -> Vec<mavlink::common::MavMessage> {
        let context = self.context.lock().await;
        let mut result = Vec::new();

        match context.registry.commands.all_vehicle_commands().await {
            Ok(commands) => {
                for command in commands {
                    match context.mav_id_from_vehicle_id(&command.vehicle_id){
                        Some(mav_id) => {
                            if let Some(msg) = self.process_command(command, mav_id) {
                                result.push(msg);
                            }
                        },
                        None => {
                            log::warn!("Vehicle not found for command: {:?}", command);
                            continue;
                        }
                    }
                }
            },
            Err(err) => {
                log::error!("Error getting all vehicle commands: {}", err);
            },
        }
        result
    }

    // TODO: loop listening for commands & cancel commands
    pub async fn handle_message(&mut self, header: &MavHeader, msg: &MavMessage) {
        // TODO: handle acts
    }
}