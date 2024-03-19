use std::{collections::HashMap, sync::Arc};

use crate::{context::AppContext, models::vehicles::{VehicleDescription, VehicleId}};
use crate::persistence::{communication, vehicles};

pub struct MavlinkContext {
    pub communication: Arc<communication::Persistence>,
    pub vehicles: Arc<vehicles::Persistence>,
    pub mav_vehicles: HashMap<u8, VehicleDescription>,
    pub auto_add_vehicles: bool
}

impl MavlinkContext {
    pub fn new(context: AppContext) -> Self {
        Self {
            communication: context.communication,
            vehicles: context.vehicles,
            mav_vehicles: HashMap::new(),
            auto_add_vehicles: true // TODO: to settings
        }
    }

    pub fn vehicle_id_from_mav_id(&self, mav_id: &u8) -> Option<VehicleId>{
        match self.mav_vehicles.get(mav_id) {
            Some(vehicle) => Some(vehicle.id.clone()),
            None => None,
        }
    }
}
