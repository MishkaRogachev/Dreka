use mavlink::{MavHeader, common::{MavMessage, ATTITUDE_DATA, ALTITUDE_DATA, VFR_HUD_DATA, GLOBAL_POSITION_INT_DATA}};

use super::{context::MavlinkContext, utils};

pub async fn handle_message(context: &mut MavlinkContext, header: &MavHeader, msg: &MavMessage) {
    match msg {
        MavMessage::ATTITUDE(attitude) =>
            handle_attitude(context, &header.system_id, attitude).await,
        MavMessage::ALTITUDE(altitude) =>
            handle_altitude(context, &header.system_id, altitude).await,
        MavMessage::VFR_HUD(vfr_hud) =>
            handle_vfr_hud(context, &header.system_id, vfr_hud).await,
        MavMessage::GLOBAL_POSITION_INT(global_position) =>
            handle_global_position(context, &header.system_id, global_position).await,
        _ => {}
    }
}

pub async fn handle_attitude(context: &mut MavlinkContext, mav_id: &u8, attitude: &ATTITUDE_DATA) {
    let mut telemetry = context.telemetry_for_mav(mav_id);

    telemetry.flight.pitch = utils::decode_angles(attitude.pitch);
    telemetry.flight.roll = utils::decode_angles(attitude.roll);
    telemetry.flight.yaw = utils::decode_angles(attitude.yaw);
    telemetry.flight.timestamp = chrono::prelude::Utc::now().timestamp_millis();

    context.set_telemetry_for_mav(mav_id, telemetry);
}

pub async fn handle_altitude(context: &mut MavlinkContext, mav_id: &u8, altitude: &ALTITUDE_DATA) {
    let mut telemetry = context.telemetry_for_mav(mav_id);

    telemetry.altitude.reference_altitude = altitude.altitude_monotonic;
    telemetry.altitude.altitude_amsl = altitude.altitude_amsl;
    telemetry.altitude.altitude_relative = altitude.altitude_relative;
    telemetry.altitude.altitude_terrain = altitude.altitude_terrain;
    telemetry.altitude.bottom_clearance = altitude.bottom_clearance;
    telemetry.altitude.timestamp = chrono::prelude::Utc::now().timestamp_millis();

    context.set_telemetry_for_mav(mav_id, telemetry);
}

pub async fn handle_vfr_hud(context: &mut MavlinkContext, mav_id: &u8, vfr_hud: &VFR_HUD_DATA) {
    let mut telemetry = context.telemetry_for_mav(mav_id);

    telemetry.flight.indicated_airspeed = vfr_hud.airspeed;
    telemetry.flight.true_airspeed = utils::to_true_airspeed(vfr_hud.airspeed, vfr_hud.alt);
    telemetry.flight.ground_speed = vfr_hud.groundspeed;
    telemetry.flight.climb = vfr_hud.climb;
    telemetry.flight.altitude_amsl = vfr_hud.alt;
    telemetry.flight.throttle = vfr_hud.throttle;
    telemetry.flight.timestamp = chrono::prelude::Utc::now().timestamp_millis();

    context.set_telemetry_for_mav(mav_id, telemetry);
}

pub async fn handle_global_position(context: &mut MavlinkContext, mav_id: &u8, global_pos: &GLOBAL_POSITION_INT_DATA) {
    let mut telemetry = context.telemetry_for_mav(mav_id);

    telemetry.flight.position.latitude = utils::decode_lat_lon(global_pos.lat);
    telemetry.flight.position.longitude = utils::decode_lat_lon(global_pos.lat);
    telemetry.flight.position.altitude = utils::decode_altitude(global_pos.lat);
    telemetry.flight.position.frame = crate::models::spatial::GeodeticFrame::Wgs84AboveSeaLevel;

    context.set_telemetry_for_mav(mav_id, telemetry);
}
