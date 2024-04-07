use mavlink::common::*;
use crate::models::{missions::*, spatial::*};
use super::super::telemetry::protocol::*;

pub fn mission_request_list(mav_id: &u8) -> MavMessage {
    log::info!("SEND REQUEST ITEM to MAVLink {}", mav_id);
    return MavMessage::MISSION_REQUEST_LIST(MISSION_REQUEST_LIST_DATA {
        target_system: mav_id.clone(),
        target_component: MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
        // mission_type: MavMissionType::MAV_MISSION_TYPE_MISSION
    })
}

pub fn request_mission_item(mav_id: &u8, seq: u16) -> MavMessage {
    log::info!("SEND REQUEST ITEM {} to MAVLink {}", seq, mav_id);
    return MavMessage::MISSION_REQUEST_INT(MISSION_REQUEST_INT_DATA {
        seq,
        target_system: mav_id.clone(),
        target_component: MavComponent::MAV_COMP_ID_MISSIONPLANNER as u8,
        // mission_type: MavMissionType::MAV_MISSION_TYPE_MISSION,
    });
}

fn yaw_from_param(param: f32) -> Option<u16> {
    if param.is_nan() {
        return Option::None
    } else {
        return Option::Some(param as u16)
    }
}

fn position_from_mavlink(item_data: &MISSION_ITEM_INT_DATA) -> Geodetic {
    return Geodetic {
        latitude: decode_lat_lon(item_data.x),
        longitude: decode_lat_lon(item_data.y),
        altitude: item_data.z,
        frame: {
            match item_data.frame {
                MavFrame::MAV_FRAME_GLOBAL => GeodeticFrame::Wgs84AboveSeaLevel,
                MavFrame::MAV_FRAME_GLOBAL_RELATIVE_ALT => GeodeticFrame::Wgs84RelativeHome,
                MavFrame::MAV_FRAME_GLOBAL_TERRAIN_ALT => GeodeticFrame::Wgs84AboveTerrain,
                _ => GeodeticFrame::None,
            }
        }
    }
}

pub fn mission_route_item_from_mavlink(item_data: &MISSION_ITEM_INT_DATA) -> MissionRouteItem {
    match item_data.command {
        MavCmd::MAV_CMD_NAV_WAYPOINT => {
            return MissionRouteItem::Waypoint {
                position: position_from_mavlink(item_data),
                hold: item_data.param1 as u16,
                pass_radius: item_data.param2,
                accept_radius: item_data.param3,
                yaw: yaw_from_param(item_data.param4)
            };
        },
        MavCmd::MAV_CMD_NAV_LOITER_TURNS => {
            return MissionRouteItem::LoiterTrn {
                position: position_from_mavlink(item_data),
                turns: item_data.param1 as u16, 
                heading_required: item_data.param2 != 0.0,
                radius: item_data.param3.abs(),
                clockwise: item_data.param3 > 0.0
            };
        },
        MavCmd::MAV_CMD_NAV_LOITER_TO_ALT => {
            return MissionRouteItem::LoiterAlt {
                position: position_from_mavlink(item_data),
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
                position: position_from_mavlink(item_data),
                abort_altitude: if item_data.param1 == 0.0 { Option::None } else { Option::Some(item_data.param1) }, 
                yaw: yaw_from_param(item_data.param4)
            }
        },
        MavCmd::MAV_CMD_NAV_TAKEOFF => {
            return MissionRouteItem::Takeoff {
                position: position_from_mavlink(item_data),
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