use mavlink::{MavHeader, common::MavMessage};

use crate::models::vehicles;

use super::context::MavlinkContext;

pub async fn handle_message(context: &mut MavlinkContext, header: &MavHeader, msg: &MavMessage) {
    if let MavMessage::HEARTBEAT(heartbeat_data) = msg {

        println!("heartbeat");
        if let Some(vehicle) = context.obtain_vehicle(header.system_id).await {
            println!("vehicle: {:?}", &vehicle);
        }
    }
}