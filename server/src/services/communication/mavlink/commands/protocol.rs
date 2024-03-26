use mavlink::common::{MavMessage, MavCmd};
use mavlink::common::{COMMAND_LONG_DATA, MISSION_SET_CURRENT_DATA, MISSION_ITEM_DATA, RC_CHANNELS_OVERRIDE_DATA};

use crate::models::spatial::Geodetic;

pub fn arm_disarm(mav_id: u8, arm: bool, attempt: u8) -> MavMessage {
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

pub fn set_waypoint(mav_id: u8, wp: u16) -> MavMessage {
    log::info!("Mav: {} Set Waypoint: {}", mav_id, wp);
    MavMessage::MISSION_SET_CURRENT(MISSION_SET_CURRENT_DATA{
        seq: wp,
        target_system: mav_id,
        target_component: 0,
    })
}

pub fn nav_to(mav_id: u8, position: &Geodetic) -> MavMessage {
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

pub fn takeoff(mav_id: u8, altitude: f32, attempt: u8) -> MavMessage {
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

pub fn go_around(mav_id: u8, attempt: u8) -> MavMessage {
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

pub fn set_servo(mav_id: u8, channel: u16, value: u16, attempt: u8) -> MavMessage {
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

pub fn override_servos(mav_id: u8, servos: &std::collections::BTreeMap<u16, u16>) -> MavMessage {
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
