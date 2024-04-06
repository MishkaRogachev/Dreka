
use std::sync::Arc;
use tokio::{sync::Mutex, sync::broadcast::Receiver};
use mavlink::{MavHeader, common::*};

use crate::models::missions::*;
use crate::models::events::ClientEvent;
use super::{super::context::MavlinkContext, protocol};

pub struct MissionHandler {
    context: Arc<Mutex<MavlinkContext>>,
    client_events_rx: Receiver<ClientEvent>
}

impl MissionHandler {
    pub fn new(context: Arc<Mutex<MavlinkContext>>, client_events_rx: Receiver<ClientEvent>) -> Self {
        Self { context, client_events_rx }
    }

    async fn handle_client_event(&mut self, event: ClientEvent) {
        match event {
            ClientEvent::DownloadMission { mission_id } => {
                log::info!("Download mission: {:?}", &mission_id);
            }
            ClientEvent::UploadMission { mission_id } => {
                log::info!("Upload mission: {:?}", &mission_id);
            }
            ClientEvent::ClearMission { mission_id } => {
                log::info!("Clear mission: {:?}", &mission_id);
            }
            ClientEvent::CancelMissionState { mission_id } => {
                log::info!("Cancel mission state: {:?}", &mission_id);
            }
            _ => {}
        }
    }

    async fn collect_states_messages(&mut self) -> Vec<MavMessage> {
        let mut messages = Vec::new();
        return messages;
    }

    pub async fn prepare_messages(&mut self) -> Vec<MavMessage> {
        match self.client_events_rx.try_recv() {
            Ok(event) => self.handle_client_event(event).await,
            Err(err) => {
                if err != tokio::sync::broadcast::error::TryRecvError::Empty {
                    log::error!("RX error: {}", err);
                }
            }
        }
        self.collect_states_messages().await
    }

    pub async fn handle_message(&mut self, header: &MavHeader, msg: &MavMessage) {
        match msg {
        // TODO: implement
            _ => {}
        }
    }
}