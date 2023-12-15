use std::sync::Arc;
use mavlink::{common::MavMessage, MavHeader};
use tokio::sync::Mutex;

use super::context::MavlinkContext;
pub struct CommandHandler {
    context: Arc<Mutex<MavlinkContext>>,
}

impl CommandHandler {
    pub fn new(context: Arc<Mutex<MavlinkContext>>) -> Self {
        Self {
            context
        }
    }

    // TODO: loop listening for commands & cancle commands

    pub async fn handle_message(&mut self, header: &MavHeader, msg: &MavMessage) {
        // TODO: handle acts
    }

    pub async fn done(&mut self) {
        // TODO: done
    }
}