use mavlink::common::*;

use crate::models::commands::Command;
use crate::models::spatial::Geodetic;

fn arm_disarm(mav_id: u8, arm: bool, attempt: u8) -> MavMessage {
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

pub fn set_mode(mav_id: u8, mode: u32, attempt: u8) -> MavMessage {
    log::info!("Mav: {} SetMode: {}", mav_id, mode);
    MavMessage::COMMAND_LONG(COMMAND_LONG_DATA{
        param1: 1.0,
        param2: mode as f32,
        param3: 0.0,
        param4: 0.0,
        param5: 0.0,
        param6: 0.0,
        param7: 0.0,
        command: MavCmd::MAV_CMD_DO_SET_MODE,
        target_system: mav_id,
        target_component: 0,
        confirmation: attempt,
    })
}

fn nav_to(mav_id: u8, position: &Geodetic) -> MavMessage {
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

fn takeoff(mav_id: u8, altitude: f32, attempt: u8) -> MavMessage {
    log::info!("Mav: {} Takeoff: {}", mav_id, altitude);
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

fn go_around(mav_id: u8, attempt: u8) -> MavMessage {
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

fn set_servo(mav_id: u8, channel: u16, value: u16, attempt: u8) -> MavMessage {
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

fn override_servos(mav_id: u8, servos: &std::collections::BTreeMap<u16, u16>) -> MavMessage {
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

pub struct EncodedCommand {
    pub message: MavMessage,
    pub ack_cmd: Option<MavCmd>,
}

pub fn encode_set_mode(mode: u32, mav_id: u8, attempt: u8) -> EncodedCommand {
    EncodedCommand {
        message: set_mode(mav_id, mode, attempt),
        ack_cmd: Some(MavCmd::MAV_CMD_DO_SET_MODE),
    }
}

pub fn encode_command(command: &Command, mav_id: u8, attempt: u8) -> Option<EncodedCommand> {
    match command {
        Command::ArmDisarm { arm } => Some(EncodedCommand {
            message: arm_disarm(mav_id, *arm, attempt),
            ack_cmd: Some(MavCmd::MAV_CMD_COMPONENT_ARM_DISARM),
        }),
        Command::NavTo { position } => Some(EncodedCommand {
            message: nav_to(mav_id, position),
            ack_cmd: Some(MavCmd::MAV_CMD_NAV_WAYPOINT),
        }),
        Command::Takeoff { altitude } => Some(EncodedCommand {
            message: takeoff(mav_id, *altitude, attempt),
            ack_cmd: Some(MavCmd::MAV_CMD_NAV_TAKEOFF),
        }),
        Command::GoAround {} => Some(EncodedCommand {
            message: go_around(mav_id, attempt),
            ack_cmd: Some(MavCmd::MAV_CMD_DO_GO_AROUND),
        }),
        Command::SetServo { channel, value } => Some(EncodedCommand {
            message: set_servo(mav_id, *channel, *value, attempt),
            ack_cmd: Some(MavCmd::MAV_CMD_DO_SET_SERVO),
        }),
        Command::OverrideServos { servos } => Some(EncodedCommand {
            message: override_servos(mav_id, &servos),
            ack_cmd: None,
        }),
        _ => None
    }
}
