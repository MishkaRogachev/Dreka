use std::{sync::Arc, collections::HashMap};

use crate::datasource::db;
use crate::models::vehicles::VehicleDescription;

pub struct MavlinkContext {
    pub repository: Arc<db::Repository>,
    pub mav_vehicles: HashMap<u8, VehicleDescription>,
    pub auto_add_vehicles: bool
}

impl MavlinkContext {
    pub fn new(repository: Arc<db::Repository>) -> Self {
        Self {
            repository,
            mav_vehicles: HashMap::new(),
            auto_add_vehicles: true // TODO: to settings
        }
    }
}
