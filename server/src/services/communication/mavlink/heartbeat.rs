use std::sync::Arc;
use tokio::sync::Mutex;

use mavlink::{MavHeader, common::{MavMessage, MavType, MavState}};

use crate::models::vehicles::{VehicleStatus, VehicleType, VehicleState, VehicleDescription, ProtocolId};
use super::context::MavlinkContext;

impl VehicleType {
    pub fn from_mavlink(mavtype: MavType) -> VehicleType {
        match mavtype {
            MavType::MAV_TYPE_FIXED_WING | MavType::MAV_TYPE_KITE | MavType::MAV_TYPE_FLAPPING_WING =>
                return VehicleType::FixedWing,
            MavType::MAV_TYPE_TRICOPTER | MavType::MAV_TYPE_QUADROTOR | MavType::MAV_TYPE_HEXAROTOR | MavType::MAV_TYPE_OCTOROTOR =>
                return VehicleType::Copter,
            MavType::MAV_TYPE_COAXIAL | MavType::MAV_TYPE_HELICOPTER =>
                return VehicleType::RotaryWing,
            MavType::MAV_TYPE_VTOL_FIXEDROTOR | MavType::MAV_TYPE_VTOL_TAILSITTER | MavType::MAV_TYPE_VTOL_TILTWING | MavType::MAV_TYPE_VTOL_TILTROTOR |
            MavType::MAV_TYPE_VTOL_TAILSITTER_DUOROTOR | MavType::MAV_TYPE_VTOL_TAILSITTER_QUADROTOR =>
                return VehicleType::Vtol,
            MavType::MAV_TYPE_AIRSHIP | MavType::MAV_TYPE_FREE_BALLOON =>
            return VehicleType::Airship,
            _ => return VehicleType::Unknown,
        }
    }
}

impl VehicleState {
    pub fn from_mavlink(system_status: MavState) -> VehicleState {
        match system_status {
            MavState::MAV_STATE_UNINIT => VehicleState::Init,
            MavState::MAV_STATE_BOOT => VehicleState::Boot,
            MavState::MAV_STATE_CALIBRATING => VehicleState::Calibrating,
            MavState::MAV_STATE_STANDBY => VehicleState::Standby,
            MavState::MAV_STATE_ACTIVE => VehicleState::Active,
            MavState::MAV_STATE_CRITICAL => VehicleState::Critical,
            MavState::MAV_STATE_EMERGENCY => VehicleState::Emergency,
            MavState::MAV_STATE_POWEROFF => VehicleState::PowerOff,
            MavState::MAV_STATE_FLIGHT_TERMINATION => VehicleState::FlightTermination,
        }
    }
}

pub struct HeartbeatHandler {
    context: Arc<Mutex<MavlinkContext>>
}

impl HeartbeatHandler {
    pub fn new(context: Arc<Mutex<MavlinkContext>>) -> Self {
        Self { context }
    }

    pub async fn handle_message(&mut self, header: &MavHeader, msg: &MavMessage) {
        if let MavMessage::HEARTBEAT(heartbeat_data) = msg {
            if let Some(mut vehicle) = self.obtain_vehicle(header.system_id).await {
                let context = self.context.lock().await;
                // Chanage type if auto
                if vehicle.vehicle_type == VehicleType::Auto {
                    vehicle.vehicle_type = VehicleType::from_mavlink(heartbeat_data.mavtype);
                    let saved = context.repository.upsert("vehicle_descriptions", &vehicle).await;
                    if let Err(err) = saved {
                        println!("Save vehicle description error: {:?}", &err);
                    }
                }

                // Save vehicle status
                let status = VehicleStatus {
                    id: vehicle.id.unwrap(),
                    last_heartbeat: chrono::prelude::Utc::now().timestamp_millis(),
                    state: VehicleState::from_mavlink(heartbeat_data.system_status)
                };
                let saved = context.repository.upsert("vehicle_statuses", &status).await;
                if let Err(err) = saved {
                    println!("Save vehicle status error: {:?}", &err);
                }

                // TODO: vehicle modes
                // TODO: vehicle flags
            }
        }
    }

    async fn obtain_vehicle(&mut self, mav_id: u8) -> Option<VehicleDescription> {
        let mut context = self.context.lock().await;

        let protocol_id = ProtocolId::MavlinkId { mav_id: mav_id };
        match context.repository.read_where::<VehicleDescription, ProtocolId>(
            "vehicle_descriptions", "protocol_id", &protocol_id).await {
            Ok(vehicle) => {
                context.mav_vehicles.insert(mav_id, vehicle.clone());
                return Some(vehicle);
            },
            Err(err) => {
                if let crate::datasource::db::DbError::NoData = err {
                    // skip & create instead
                } else {
                    println!("Read vehicle error : {}", &err);
                }
            }
        }

        if context.auto_add_vehicles {
            let result = context.repository.create("vehicles", &VehicleDescription {
                id: None,
                protocol_id: protocol_id,
                name: format!("Nev Vehicle (MAV {})", mav_id).into(),
                vehicle_type: VehicleType::Auto,
                features: Vec::new()
            }).await;
            match result {
                Ok(vehicle) => {
                    context.mav_vehicles.insert(mav_id, vehicle.clone());
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