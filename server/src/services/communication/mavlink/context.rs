use std::{sync::Arc, collections::HashMap};
use tokio::time;

use crate::{datasource::db, models::vehicles::{VehicleDescription, VehicleType, ProtocolId}};

pub struct MavlinkContext {
    repository: Arc<db::Repository>,
    mav_vehicles: HashMap<u8, VehicleDescription>,
    mav_timers: HashMap<u8, time::Instant>,
    auto_add_vehicles: bool
}

impl MavlinkContext {
    pub fn new(repository: Arc<db::Repository>) -> Self {
        Self {
            repository,
            mav_vehicles: HashMap::new(),
            mav_timers: HashMap::new(),
            auto_add_vehicles: true // TODO: to settings
        }
    }

    pub async fn get_vehicle(&self, mav_id: u8) -> Option<VehicleDescription> {
        match self.mav_vehicles.get(&mav_id) {
            Some(vehicle) => Some(vehicle.clone()),
            None => None,
        }
    }

    pub async fn obtain_vehicle(&mut self, mav_id: u8) -> Option<VehicleDescription> {
        let vehicle = self.get_vehicle(mav_id).await;
        if vehicle.is_some() {
            return vehicle;
        }

        let protocol_id = ProtocolId::MavlinkId { mav_id: mav_id };
        match self.repository.read_where::<VehicleDescription, ProtocolId>(
            "vehicle_descriptions", "protocol_id", &protocol_id).await {
            Ok(vehicle) => {
                return self.mav_vehicles.insert(mav_id, vehicle);
            },
            Err(err) => {
                if let db::DbError::NoData = err {
                    // skip
                } else {
                    println!("Read vehicle error : {}", &err);
                }
            }
        }

        if self.auto_add_vehicles {
            let result = self.repository.create("vehicles", &VehicleDescription {
                id: None,
                protocol_id: protocol_id,
                name: format!("Nev Vehicle (MAV {})", mav_id).into(),
                vehicle_type: VehicleType::Auto,
                features: Vec::new()
            }).await;
            match result {
                Ok(vehicle) => {
                    return self.mav_vehicles.insert(mav_id, vehicle);
                },
                Err(err) => {
                    println!("Insert vehicle error : {}", &err);
                }
            }
        }
        return None;
    }
}
