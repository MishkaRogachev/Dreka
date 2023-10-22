use mavlink::{MavHeader, common::MavMessage};

use super::context::MavlinkContext;

pub async fn handle_message(context: &MavlinkContext, header: &MavHeader, msg: &MavMessage) {

}
