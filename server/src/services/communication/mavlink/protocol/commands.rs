use mavlink::common::*;

use crate::models::commands::{Calibration, Command};
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

fn calibrate(mav_id: u8, calibration: Calibration, attempt: u8) -> MavMessage {
    log::info!("Mav: {} Calibrate: {:?}", mav_id, calibration);
    MavMessage::COMMAND_LONG(COMMAND_LONG_DATA{
        param1: if calibration == Calibration::Temperature {
            3.0
        } else {
            0.0
        },
        param2: 0.0,
        param3: if calibration == Calibration::GroundPressure {
            1.0
        } else {
            0.0
        },
        param4: 0.0,
        param5: if calibration == Calibration::Temperature {
            3.0
        } else {
            0.0
        },
        param6: if calibration == Calibration::Airspeed {
            2.0
        } else {
            0.0
        },
        param7: if calibration == Calibration::Temperature {
            3.0
        } else {
            0.0
        },
        command: MavCmd::MAV_CMD_PREFLIGHT_CALIBRATION,
        target_system: mav_id,
        target_component: mavlink::common::MavComponent::MAV_COMP_ID_ALL as u8,
        confirmation: attempt,
    })
}

fn set_home(mav_id: u8, position: Geodetic) -> MavMessage {
    log::info!("Mav: {} SetHome: {:?}", mav_id, position);
    let (frame, x, y, z) = position.to_mavlink();
    MavMessage::COMMAND_INT(COMMAND_INT_DATA{
        param1: 0.0,
        param2: 0.0,
        param3: 0.0,
        param4: 0.0,
        command: MavCmd::MAV_CMD_DO_SET_HOME,
        current: 0,
        autocontinue: 0,
        x,
        y,
        z,
        frame,
        target_system: mav_id,
        target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
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

pub fn set_waypoint(mav_id: u8, wp: u16, attempt: u8) -> MavMessage {
    log::info!("Mav: {} SetWaypoint: {}", mav_id, wp);
    MavMessage::COMMAND_LONG(COMMAND_LONG_DATA{
        param1: wp as f32,
        param2: 0.0,
        param3: 0.0,
        param4: 0.0,
        param5: 0.0,
        param6: 0.0,
        param7: 0.0,
        command: MavCmd::MAV_CMD_DO_SET_MISSION_CURRENT,
        target_system: mav_id,
        target_component: mavlink::common::MavComponent::MAV_COMP_ID_ALL as u8,
        confirmation: attempt,
    })
}

fn nav_to(mav_id: u8, position: Geodetic) -> MavMessage {
    log::info!("Mav: {} Nav to: {:?}", mav_id, position);
    let (frame, x, y, z) = position.to_mavlink();
    MavMessage::COMMAND_INT(COMMAND_INT_DATA{
        param1: -1.0, // Ground speed in m/s
        param2: 0.0, // TODO: MAV_DO_REPOSITION_FLAGS_CHANGE_MODE
        param3: 0.0, // TODO: MAV_DO_REPOSITION_FLAGS_CONTINUE
        param4: 0.0, // TODO: Yaw heading. NaN to use the current system yaw heading mode (e.g. yaw towards next waypoint, yaw to home, etc.). For planes indicates loiter direction (0: clockwise, 1: counter clockwise)
        command: mavlink::common::MavCmd::MAV_CMD_DO_REPOSITION,
        current: 2, // guided
        autocontinue: 0,
        x,
        y,
        z,
        frame,
        target_system: mav_id,
        target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
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

fn land(mav_id: u8, position: Geodetic, abort_altitude: f32) -> MavMessage {
    log::info!("Mav: {} Land", mav_id);
    let (frame, x, y, z) = position.to_mavlink();
    MavMessage::COMMAND_INT(COMMAND_INT_DATA{
        param1: abort_altitude,
        param2: 0.0, // TODO: precision landing
        param3: 0.0,
        param4: f32::NAN, // Desired yaw angle. NaN to use the current system yaw heading mode (e.g. yaw towards next waypoint, yaw to home, etc.).
        command: MavCmd::MAV_CMD_NAV_LAND,
        current: 0,
        autocontinue: 0,
        x,
        y,
        z,
        frame,
        target_system: mav_id,
        target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
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

fn override_servos(mav_id: u8, servos: std::collections::BTreeMap<u16, u16>) -> MavMessage {
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

pub fn encode_command(command: Command, mav_id: u8, attempt: u8) -> Option<EncodedCommand> {
    match command {
        Command::ArmDisarm { arm } => Some(EncodedCommand {
            message: arm_disarm(mav_id, arm, attempt),
            ack_cmd: Some(MavCmd::MAV_CMD_COMPONENT_ARM_DISARM),
        }),
        Command::Calibrate { calibration } => Some(EncodedCommand {
            message: calibrate(mav_id, calibration, attempt),
            ack_cmd: Some(MavCmd::MAV_CMD_PREFLIGHT_CALIBRATION),
        }),
        Command::SetHome { position } => Some(EncodedCommand {
            message: set_home(mav_id, position),
            ack_cmd: Some(MavCmd::MAV_CMD_DO_SET_HOME),
        }),
        Command::SetWaypoint { wpt } => Some(EncodedCommand {
            message: set_waypoint(mav_id, wpt, attempt),
            ack_cmd: Some(MavCmd::MAV_CMD_DO_SET_MISSION_CURRENT),
        }),
        Command::NavTo { position } => Some(EncodedCommand {
            message: nav_to(mav_id, position),
            ack_cmd: Some(MavCmd::MAV_CMD_NAV_WAYPOINT),
        }),
        Command::Takeoff { altitude } => Some(EncodedCommand {
            message: takeoff(mav_id, altitude, attempt),
            ack_cmd: Some(MavCmd::MAV_CMD_NAV_TAKEOFF),
        }),
        Command::Land { position, abort_altitude } => Some(EncodedCommand {
            message: land(mav_id, position, abort_altitude.unwrap_or(0.0)),
            ack_cmd: Some(MavCmd::MAV_CMD_NAV_LAND),
        }),
        Command::GoAround {} => Some(EncodedCommand {
            message: go_around(mav_id, attempt),
            ack_cmd: Some(MavCmd::MAV_CMD_DO_GO_AROUND),
        }),
        Command::SetServo { channel, value } => Some(EncodedCommand {
            message: set_servo(mav_id, channel, value, attempt),
            ack_cmd: Some(MavCmd::MAV_CMD_DO_SET_SERVO),
        }),
        Command::OverrideServos { servos } => Some(EncodedCommand {
            message: override_servos(mav_id, servos),
            ack_cmd: None,
        }),
        _ => None
    }
}
