use std::{sync::Arc, collections::HashMap};

use crate::datasource::db;
use crate::models::telemetry::{FlightData, SnsData, AltitudeData};
use crate::models::vehicles::{VehicleDescription, VehicleType, ProtocolId};

#[derive(Clone)]
pub struct Telemetry {
    pub flight: FlightData,
    pub altitude: AltitudeData,
    pub sns: SnsData
}

pub struct MavlinkContext {
    pub repository: Arc<db::Repository>,
    pub mav_vehicles: HashMap<u8, VehicleDescription>,
    pub telemetry: HashMap<u8, Telemetry>,
    auto_add_vehicles: bool
}

impl Default for Telemetry {
    fn default() -> Telemetry {
        Telemetry {
            flight: FlightData::default(),
            altitude: AltitudeData::default(),
            sns: SnsData::default()
        }
    }
}

impl MavlinkContext {
    pub fn new(repository: Arc<db::Repository>) -> Self {
        Self {
            repository,
            mav_vehicles: HashMap::new(),
            telemetry: HashMap::new(),
            auto_add_vehicles: true // TODO: to settings
        }
    }

    pub fn telemetry_for_mav(&self, mav_id: &u8) -> Telemetry {
        let telemetry = self.telemetry.get(mav_id);
        if let Some(telemetry) = telemetry {
            return telemetry.to_owned();
        }

        return Telemetry::default();
    }

    pub fn set_telemetry_for_mav(&mut self, mav_id: &u8, telemetry: Telemetry) {
        self.telemetry.insert(*mav_id, telemetry);
    }

    pub async fn obtain_vehicle(&mut self, mav_id: u8) -> Option<VehicleDescription> {
        let vehicle = self.mav_vehicles.get(&mav_id);
        if let Some(vehicle) = vehicle {
            return Some(vehicle.to_owned());
        }

        let protocol_id = ProtocolId::MavlinkId { mav_id: mav_id };
        match self.repository.read_where::<VehicleDescription, ProtocolId>(
            "vehicle_descriptions", "protocol_id", &protocol_id).await {
            Ok(vehicle) => {
                self.mav_vehicles.insert(mav_id, vehicle.clone());
                return Some(vehicle);
            },
            Err(err) => {
                if let db::DbError::NoData = err {
                    // skip & create instead
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
                    self.mav_vehicles.insert(mav_id, vehicle.clone());
                    return Some(vehicle);
                },
                Err(err) => {
                    println!("Insert vehicle error : {}", &err);
                }
            }
        }
        return None;
    }
}
