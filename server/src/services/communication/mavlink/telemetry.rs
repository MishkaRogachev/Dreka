use mavlink::{MavHeader, common::{MavMessage, ATTITUDE_DATA, ALTITUDE_DATA}};

use super::{context::MavlinkContext, utils};

pub async fn handle_message(context: &mut MavlinkContext, header: &MavHeader, msg: &MavMessage) {
    match msg {
        MavMessage::ATTITUDE(attitude_data) =>
            handle_attitude(context, &header.system_id, attitude_data).await,
        MavMessage::ALTITUDE(altitude_data) =>
            handle_altitude(context, &header.system_id, altitude_data).await,
        _ => {}
    }

    // TODO: flush telemetry
}

pub async fn handle_attitude(context: &mut MavlinkContext, mav_id: &u8, attitude_data: &ATTITUDE_DATA) {
    let mut telemetry = context.telemetry_for_mav(mav_id);

    telemetry.flight.pitch = utils::decode_angles(attitude_data.pitch);
    telemetry.flight.roll = utils::decode_angles(attitude_data.roll);
    telemetry.flight.yaw = utils::decode_angles(attitude_data.yaw);

    telemetry.flight.timestamp = chrono::prelude::Utc::now().timestamp_millis();

    context.set_telemetry_for_mav(mav_id, telemetry);
}

pub async fn handle_altitude(context: &mut MavlinkContext, mav_id: &u8, altitude_data: &ALTITUDE_DATA) {
    let mut telemetry = context.telemetry_for_mav(mav_id);

    telemetry.altitude.reference_altitude = altitude_data.altitude_monotonic;
    telemetry.altitude.altitude_amsl = altitude_data.altitude_amsl;
    telemetry.altitude.altitude_relative = altitude_data.altitude_relative;

    telemetry.altitude.altitude_terrain = altitude_data.altitude_terrain;
    telemetry.altitude.bottom_clearance = altitude_data.bottom_clearance;

    telemetry.flight.timestamp = chrono::prelude::Utc::now().timestamp_millis();

    context.set_telemetry_for_mav(mav_id, telemetry);
}
