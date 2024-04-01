use std::sync::Arc;
use tokio::sync::Mutex;

use mavlink::{common::{MavAutopilot, MavMessage, MavModeFlag, MavState, MavType}, MavHeader};

use crate::models::{colors::EntityColor, vehicles::*};
use super::{super::context::MavlinkContext, protocol};

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
            let vehicle = self.obtain_vehicle(header.system_id).await;
            if let Err(err) = vehicle {
                log::error!("Obtain vehicle error: {:?}", &err);
                return;
            }

            match vehicle.unwrap() {
                Some(mut vehicle) => {
                    let mut save_vehicle: bool = false;
                    let context = self.context.lock().await;
                    // Chanage type if auto
                    if vehicle.vehicle_type == VehicleType::Auto {
                        vehicle.vehicle_type = VehicleType::from_mavlink(heartbeat_data.mavtype);
                        save_vehicle = true;
                    }

                    let mode: VehicleMode;
                    let available_modes: Vec<VehicleMode>;
                    match heartbeat_data.autopilot {
                        MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA => {
                            mode = protocol::decode_apm_mode(heartbeat_data.mavtype, heartbeat_data.custom_mode);
                            available_modes = protocol::available_apm_modes(heartbeat_data.mavtype);
                        },
                        // TODO: px4 modes
                        _ => {
                            mode = VehicleMode::None;
                            available_modes = Vec::new();
                        }
                    }

                    if vehicle.available_modes != available_modes {
                        vehicle.available_modes = available_modes;
                        save_vehicle = true;
                    }

                    if save_vehicle {
                        if let Err(err) = context.registry.vehicles.save_vehicle(&vehicle).await {
                            log::error!("Save vehicle description error: {:?}", &err);
                        }
                    }

                    // TODO: vehicle 
                    // MAV_MODE_FLAG_MANUAL_INPUT_ENABLED, MAV_MODE_FLAG_STABILIZE_ENABLED, MAV_MODE_FLAG_GUIDED_ENABLED, MAV_MODE_FLAG_AUTO_ENABLED

                    let status = VehicleStatus {
                        id: vehicle.id,
                        last_heartbeat: chrono::prelude::Utc::now().timestamp_millis(),
                        state: VehicleState::from_mavlink(heartbeat_data.system_status),
                        armed: heartbeat_data.base_mode.intersects(MavModeFlag::MAV_MODE_FLAG_SAFETY_ARMED),
                        mode
                    };

                    // Update vehicle status in registry
                    if let Err(err) = context.registry.vehicles.update_status(&status).await {
                        log::error!("Save vehicle status error: {:?}", &err);
                    }
                },
                None => {} // Do nothing if there is no vehicle for this mavlink id
            }
        }
    }

    async fn obtain_vehicle(&mut self, mav_id: u8) -> anyhow::Result<Option<VehicleDescription>> {
        let mut context = self.context.lock().await;
        let protocol_id = ProtocolId::MavlinkId { mav_id: mav_id };
        let vehicle = context.registry.vehicles.vehicle_by_protocol_id(&protocol_id).await?;
        match vehicle {
            Some(vehicle) => {
                context.mav_vehicles.insert(mav_id, vehicle.id.clone());
                return Ok(Some(vehicle));
            },
            None => {
                if context.auto_add_vehicles {
                    // Create new vehicle and add it to registry
                    let vehicle = context.registry.vehicles.save_vehicle(&VehicleDescription {
                        id: String::new(),
                        protocol_id,
                        name: format!("New Vehicle (MAV {})", mav_id).into(),
                        color: EntityColor::Cyan,
                        vehicle_type: VehicleType::Auto,
                        features: Vec::new(),
                        available_modes: Vec::new()
                    }).await?;
                    context.mav_vehicles.insert(mav_id, vehicle.id.clone());
                    return Ok(Some(vehicle));
                }
                return Ok(None);
            }
        }
    }
}
