use std::collections::HashMap;

use crate::models::events::ServerEvent;
use crate::models::vehicles::{VehicleId, VehicleMode};
use crate::middleware::{bus, registry};

pub struct MavlinkContext {
    pub registry: registry::Registry, // TODO: move registry from context to handlers to reduce mutex locks
    pub server_bus: bus::EventBus::<ServerEvent>,
    pub mav_vehicles: HashMap<u8, VehicleId>,
    pub mav_modes: HashMap<u8, HashMap<u32, VehicleMode>>,
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
            mav_modes: HashMap::new()
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
