use std::collections::HashMap;

use crate::models::{telemetry::VehicleTelemetry, vehicles::VehicleId};
use crate::registry::registry;

pub struct MavlinkContext {
    pub registry: registry::Registry,
    pub mav_vehicles: HashMap<u8, VehicleId>,
    pub auto_add_vehicles: bool,
    telemetry_tx: flume::Sender<VehicleTelemetry>,
    telemetry_rx: flume::Receiver<VehicleTelemetry>
}

impl MavlinkContext {
    pub fn new(
        registry: registry::Registry,
        telemetry_tx: flume::Sender<VehicleTelemetry>,
        telemetry_rx: flume::Receiver<VehicleTelemetry>,
    ) -> Self {
        Self {
            registry,
            mav_vehicles: HashMap::new(),
            auto_add_vehicles: true, // TODO: to settings
            telemetry_tx,
            telemetry_rx
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

    pub fn send_telemetry(&self, telemetry: VehicleTelemetry) -> anyhow::Result<()> {
        match self.telemetry_tx.try_send(telemetry) {
            Ok(_) => { Ok(()) },
            Err(err) => match err {
                flume::TrySendError::Full(telemetry) => {
                    match self.telemetry_rx.recv() {
                        Ok(_) => self.send_telemetry(telemetry),
                        Err(err) => Err(err.into())
                    }
                }
                flume::TrySendError::Disconnected(_) => { Ok(()) }
            }
        }
    }
}
