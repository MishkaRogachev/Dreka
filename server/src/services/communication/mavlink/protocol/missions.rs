use mavlink::common::*;

use crate::models::{missions::*, spatial::*};

pub fn mission_request_list(mav_id: &u8) -> MavMessage {
    log::info!("Request mission items count from MAVLink {}", mav_id);
    return MavMessage::MISSION_REQUEST_LIST(MISSION_REQUEST_LIST_DATA {
        target_system: mav_id.clone(),
        target_component: MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
        // mission_type: MavMissionType::MAV_MISSION_TYPE_MISSION
    })
}

pub fn request_mission_item(mav_id: &u8, seq: u16) -> MavMessage {
    log::info!("Request mission item {} from MAVLink {}", seq, mav_id);
    return MavMessage::MISSION_REQUEST_INT(MISSION_REQUEST_INT_DATA {
        seq,
        target_system: mav_id.clone(),
        target_component: MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
        // mission_type: MavMissionType::MAV_MISSION_TYPE_MISSION,
    });
}

pub fn send_mission_clear(mav_id: &u8) -> MavMessage {
    log::info!("Clear all mission items on MAVLink {}", mav_id);
    return MavMessage::MISSION_CLEAR_ALL(MISSION_CLEAR_ALL_DATA {
        target_system: mav_id.clone(),
        target_component: MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
        //mission_type: MavMissionType::MAV_MISSION_TYPE_MISSION,
    });
}

pub fn send_mission_count(mav_id: &u8, count: u16) -> MavMessage {
    log::info!("Send mission items count ({}) to MAVLink {}", count, mav_id);
    return MavMessage::MISSION_COUNT(MISSION_COUNT_DATA {
        count,
        target_system: mav_id.clone(),
        target_component: MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
        // mission_type: MavMissionType::MAV_MISSION_TYPE_MISSION
    });
}

pub fn send_mission_home_item(mav_id: &u8, position: &Geodetic) -> MavMessage {
    log::info!("Send home position to MAVLink {}", mav_id);
    let (frame, x, y, z) = position.to_mavlink();
    return MavMessage::MISSION_ITEM_INT(MISSION_ITEM_INT_DATA {
        command: MavCmd::MAV_CMD_NAV_WAYPOINT,
        frame,
        x,
        y,
        z,
        param1: 0.0,
        param2: 0.0,
        param3: 0.0,
        param4: 0.0,
        seq: 0,
        //mission_type: mavlink::common::MavMissionType::MAV_MISSION_TYPE_MISSION,
        target_system: mav_id.clone(),
        target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
        current: 0,
        autocontinue: 1
    });
}

pub fn send_mission_item(mav_id: &u8, item: &MissionRouteItem, seq: u16) -> Option<MavMessage> {
    log::info!("Send mission item {} to MAVLink {}", seq, mav_id);

    match item {
        MissionRouteItem::Gap {} => {
            return Option::None;
        },
        MissionRouteItem::Waypoint { position, hold, pass_radius, accept_radius, yaw } => {
            let (frame, x, y, z) = position.to_mavlink();
            return Option::Some(MavMessage::MISSION_ITEM_INT(MISSION_ITEM_INT_DATA {
                command: MavCmd::MAV_CMD_NAV_WAYPOINT,
                frame,
                x,
                y,
                z,
                param1: *hold as f32,
                param2: *pass_radius,
                param3: *accept_radius,
                param4: yaw_to_param(*yaw),
                seq,
                //mission_type: mavlink::common::MavMissionType::MAV_MISSION_TYPE_MISSION,
                target_system: mav_id.clone(),
                target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
                current: 0,
                autocontinue: 1
            }));
        },
        MissionRouteItem::Takeoff { position, pitch, yaw } => {
            let (frame, x, y, z) = position.to_mavlink();
            return Option::Some(MavMessage::MISSION_ITEM_INT(MISSION_ITEM_INT_DATA {
                command: MavCmd::MAV_CMD_NAV_TAKEOFF,
                frame,
                x,
                y,
                z,
                param1: *pitch,
                param2: 0.0,
                param3: 0.0,
                param4: yaw_to_param(*yaw),
                seq,
                //mission_type: mavlink::common::MavMissionType::MAV_MISSION_TYPE_MISSION,
                target_system: mav_id.clone(),
                target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
                current: 0,
                autocontinue: 1
            }));
        },
        MissionRouteItem::LandStart {} => {
            return Option::Some(MavMessage::MISSION_ITEM_INT(MISSION_ITEM_INT_DATA {
                command: MavCmd::MAV_CMD_DO_LAND_START,
                frame: MavFrame::MAV_FRAME_GLOBAL,
                x: 0,
                y: 0,
                z: 0.0,
                param1: 0.0,
                param2: 0.0,
                param3: 0.0,
                param4: 0.0,
                seq,
                //mission_type: mavlink::common::MavMissionType::MAV_MISSION_TYPE_MISSION,
                target_system: mav_id.clone(),
                target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
                current: 0,
                autocontinue: 1
            }));
        },
        MissionRouteItem::Landing { position, abort_altitude, yaw } => {
            let (frame, x, y, z) = position.to_mavlink();
            return Option::Some(MavMessage::MISSION_ITEM_INT(MISSION_ITEM_INT_DATA {
                command: MavCmd::MAV_CMD_NAV_LAND,
                frame,
                x,
                y,
                z,
                param1: {
                     match *abort_altitude {
                        Some(abort_altitude) => abort_altitude,
                        None => 0.0,
                    }
                },
                param2: 0.0,
                param3: 0.0,
                param4: yaw_to_param(*yaw),
                seq,
                //mission_type: mavlink::common::MavMissionType::MAV_MISSION_TYPE_MISSION,
                target_system: mav_id.clone(),
                target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
                current: 0,
                autocontinue: 1
            }));
        },
        MissionRouteItem::LoiterTrn { position, heading_required, radius, turns, clockwise } => {
            let (frame, x, y, z) = position.to_mavlink();
            return Option::Some(MavMessage::MISSION_ITEM_INT(MISSION_ITEM_INT_DATA {
                command: MavCmd::MAV_CMD_NAV_LOITER_TURNS,
                frame,
                x,
                y,
                z,
                param1: *turns as f32,
                param2: *heading_required as i32 as f32,
                param3: if *clockwise { *radius } else { -1.0 * *radius },
                param4: 0.0,
                seq,
                //mission_type: mavlink::common::MavMissionType::MAV_MISSION_TYPE_MISSION,
                target_system: mav_id.clone(),
                target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
                current: 0,
                autocontinue: 1
            }));
        },
        MissionRouteItem::LoiterAlt { position, heading_required, radius, clockwise } => {
            let (frame, x, y, z) = position.to_mavlink();
            return Option::Some(MavMessage::MISSION_ITEM_INT(MISSION_ITEM_INT_DATA {
                command: MavCmd::MAV_CMD_NAV_LOITER_TO_ALT,
                frame,
                x,
                y,
                z,
                param1: *heading_required as i32 as f32,
                param2: if *clockwise { *radius } else { -1.0 * *radius },
                param3: 0.0,
                param4: 0.0,
                seq,
                //mission_type: mavlink::common::MavMissionType::MAV_MISSION_TYPE_MISSION,
                target_system: mav_id.clone(),
                target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
                current: 0,
                autocontinue: 1
            }));
        },
        MissionRouteItem::TriggerCam { distance, shutter, trigger }  => {
            return Option::Some(MavMessage::MISSION_ITEM_INT(MISSION_ITEM_INT_DATA {
                command: MavCmd::MAV_CMD_DO_SET_CAM_TRIGG_DIST,
                frame: MavFrame::MAV_FRAME_GLOBAL,
                x: 0,
                y: 0,
                z: 0.0,
                param1: *distance as f32,
                param2: *shutter as f32,
                param3: *trigger as i32 as f32,
                param4: 0.0,
                seq,
                //mission_type: mavlink::common::MavMissionType::MAV_MISSION_TYPE_MISSION,
                target_system: mav_id.clone(),
                target_component: mavlink::common::MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
                current: 0,
                autocontinue: 1
            }));
        }
    }
}

pub fn mission_route_item_from_mavlink(item_data: &MISSION_ITEM_INT_DATA) -> MissionRouteItem {
    match item_data.command {
        MavCmd::MAV_CMD_NAV_WAYPOINT => {
            return MissionRouteItem::Waypoint {
                position: Geodetic::from_mavlink(item_data.x, item_data.y, item_data.z, item_data.frame),
                hold: item_data.param1 as u16,
                pass_radius: item_data.param2,
                accept_radius: item_data.param3,
                yaw: yaw_from_param(item_data.param4)
            };
        },
        MavCmd::MAV_CMD_NAV_LOITER_TURNS => {
            return MissionRouteItem::LoiterTrn {
                position: Geodetic::from_mavlink(item_data.x, item_data.y, item_data.z, item_data.frame),
                turns: item_data.param1 as u16, 
                heading_required: item_data.param2 != 0.0,
                radius: item_data.param3.abs(),
                clockwise: item_data.param3 > 0.0
            };
        },
        MavCmd::MAV_CMD_NAV_LOITER_TO_ALT => {
            return MissionRouteItem::LoiterAlt {
                position: Geodetic::from_mavlink(item_data.x, item_data.y, item_data.z, item_data.frame),
                heading_required: item_data.param1 != 0.0,
                radius: item_data.param2.abs(),
                clockwise: item_data.param2 > 0.0
            };
        },
        MavCmd::MAV_CMD_DO_LAND_START => {
            return MissionRouteItem::LandStart {}
        },
        MavCmd::MAV_CMD_NAV_LAND => {
            return MissionRouteItem::Landing {
                position: Geodetic::from_mavlink(item_data.x, item_data.y, item_data.z, item_data.frame),
                abort_altitude: if item_data.param1 == 0.0 { Option::None } else { Option::Some(item_data.param1) }, 
                yaw: yaw_from_param(item_data.param4)
            }
        },
        MavCmd::MAV_CMD_NAV_TAKEOFF => {
            return MissionRouteItem::Takeoff {
                position: Geodetic::from_mavlink(item_data.x, item_data.y, item_data.z, item_data.frame),
                pitch: item_data.param1,
                yaw: yaw_from_param(item_data.param4)
            }
        },
        MavCmd::MAV_CMD_DO_SET_CAM_TRIGG_DIST => {
            return MissionRouteItem::TriggerCam {
                distance: item_data.param1,
                shutter: item_data.param2 as i16,
                trigger: item_data.param3 == 1.0,
            }
        }
        _ => return {
            log::warn!("Unsupported mission item type: {:?}", &item_data.command);
            MissionRouteItem::Gap {}
        }
    }
}

pub fn mission_home_item_from_mavlink(item_data: &MISSION_ITEM_INT_DATA) -> Geodetic {
    return Geodetic::from_mavlink(item_data.x, item_data.y, item_data.z, item_data.frame);
}

fn yaw_to_param(yaw: Option<u16>) -> f32 {
    match yaw {
        Some(yaw) => return yaw as f32,
        None => return std::f32::NAN,
    }
}

fn yaw_from_param(param: f32) -> Option<u16> {
    if param.is_nan() {
        return Option::None
    } else {
        return Option::Some(param as u16)
    }
}
