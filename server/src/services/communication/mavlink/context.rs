use std::collections::HashMap;

use crate::models::{events::ServerEvent, vehicles::VehicleId};
use crate::registry::registry;

pub struct MavlinkContext {
    pub registry: registry::Registry,
    pub mav_vehicles: HashMap<u8, VehicleId>,
    pub auto_add_vehicles: bool,
    server_events_tx: flume::Sender<ServerEvent>,
    server_events_rx: flume::Receiver<ServerEvent>
}

impl MavlinkContext {
    pub fn new(
        registry: registry::Registry,
        server_events_tx: flume::Sender<ServerEvent>,
        server_events_rx: flume::Receiver<ServerEvent>,
    ) -> Self {
        Self {
            registry,
            mav_vehicles: HashMap::new(),
            auto_add_vehicles: true, // TODO: to settings
            server_events_tx,
            server_events_rx
        }
    }

    pub fn vehicle_id_from_mav_id(&self, mav_id: &u8) -> Option<VehicleId>{
        self.mav_vehicles.get(mav_id).cloned()
    }

    pub fn mav_id_from_vehicle_id(&self, vehicle_id: &VehicleId) -> Option<u8> {
        self.mav_vehicles
            .iter()
            .find(|(_, v_id)| v_id == &vehicle_id)
            .map(|(mav_id, _)| *mav_id)
    }

    pub fn send_event(&self, telemetry: ServerEvent) -> anyhow::Result<()> {
        match self.server_events_tx.try_send(telemetry) {
            Ok(_) => { Ok(()) },
            Err(err) => match err {
                flume::TrySendError::Full(telemetry) => {
                    match self.server_events_rx.recv() {
                        Ok(_) => self.send_event(telemetry),
                        Err(err) => Err(err.into())
                    }
                }
                flume::TrySendError::Disconnected(_) => { Ok(()) }
            }
        }
    }
}
