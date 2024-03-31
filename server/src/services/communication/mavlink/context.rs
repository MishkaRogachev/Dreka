use std::collections::HashMap;

use crate::models::events::ServerEvent;
use crate::models::vehicles::VehicleId;
use crate::registry::{bus, registry};

pub struct MavlinkContext {
    pub registry: registry::Registry,
    pub server_bus: bus::EventBus::<ServerEvent>,
    pub mav_vehicles: HashMap<u8, VehicleId>,
    pub auto_add_vehicles: bool,
}

impl MavlinkContext {
    pub fn new(
        registry: registry::Registry,
        server_bus: bus::EventBus<ServerEvent>
    ) -> Self {
        Self {
            registry,
            server_bus,
            mav_vehicles: HashMap::new(),
            auto_add_vehicles: true, // TODO: to settings
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
}
