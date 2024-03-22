use std::{sync::Arc, collections::HashMap};

use mavlink::{MavHeader, common::{MavMessage, ATTITUDE_DATA, VFR_HUD_DATA, GLOBAL_POSITION_INT_DATA, GPS_RAW_INT_DATA, SYS_STATUS_DATA, MavSysStatusSensor}};
use tokio::sync::Mutex;

use super::{context::MavlinkContext, utils};
use crate::models::telemetry::{Flight, Navigation, Sensor, SensorType, System, VehicleTelemetry};

pub struct TelemetryHandler {
    context: Arc<Mutex<MavlinkContext>>,

    mav_flight_map: HashMap<u8, Flight>,
    mav_navi_map: HashMap<u8, Navigation>,
    mav_system_map: HashMap<u8, System>,
}

impl TelemetryHandler {
    pub fn new(context: Arc<Mutex<MavlinkContext>>) -> Self {
        Self {
            context,
            mav_flight_map: HashMap::new(),
            mav_navi_map: HashMap::new(),
            mav_system_map: HashMap::new(),
        }
    }

    pub async fn handle_message(&mut self, header: &MavHeader, msg: &MavMessage) {
        match msg {
            MavMessage::ATTITUDE(attitude) =>
                self.handle_attitude(header.system_id, attitude).await,
            MavMessage::VFR_HUD(vfr_hud) =>
                self.handle_vfr_hud(header.system_id, vfr_hud).await,
            MavMessage::GLOBAL_POSITION_INT(global_position) =>
                self.handle_global_position(header.system_id, global_position).await,
            MavMessage::GPS_RAW_INT(gps_raw) =>
                self.handle_gps_raw(header.system_id, gps_raw).await,
            MavMessage::SYS_STATUS(sys_data) =>
                self.handle_sys_data(header.system_id, sys_data).await,
            _ => {}
        }
    }

    pub async fn handle_attitude(&mut self, mav_id: u8, attitude: &ATTITUDE_DATA) {
        let mut flight = self.mav_flight_map.get(&mav_id).cloned().unwrap_or_default();

        flight.pitch = utils::decode_angles(attitude.pitch);
        flight.roll = utils::decode_angles(attitude.roll);
        flight.yaw = utils::decode_angles(attitude.yaw);

        self.update_flight(mav_id, flight).await;
    }

    pub async fn handle_vfr_hud(&mut self, mav_id: u8, vfr_hud: &VFR_HUD_DATA) {
        let mut flight = self.mav_flight_map.get(&mav_id).cloned().unwrap_or_default();

        flight.indicated_airspeed = vfr_hud.airspeed;
        flight.true_airspeed = utils::to_true_airspeed(vfr_hud.airspeed, vfr_hud.alt);
        flight.ground_speed = vfr_hud.groundspeed;
        flight.climb = vfr_hud.climb;
        flight.altitude_amsl = vfr_hud.alt;
        flight.throttle = vfr_hud.throttle;

        self.update_flight(mav_id, flight).await;
    }

    pub async fn handle_global_position(&mut self, mav_id: u8, global_pos: &GLOBAL_POSITION_INT_DATA) {
        let mut flight = self.mav_flight_map.get(&mav_id).cloned().unwrap_or_default();

        flight.position.latitude = utils::decode_lat_lon(global_pos.lat);
        flight.position.longitude = utils::decode_lat_lon(global_pos.lon);
        flight.position.altitude = utils::decode_altitude(global_pos.alt);
        flight.position.frame = crate::models::spatial::GeodeticFrame::Wgs84AboveSeaLevel;

        self.update_flight(mav_id, flight).await;
    }

    pub async fn handle_gps_raw(&mut self, mav_id: u8, gps_raw: &GPS_RAW_INT_DATA) { 
        let mut navi = self.mav_navi_map.get(&mav_id).cloned().unwrap_or_default();

        navi.position.latitude = utils::decode_lat_lon(gps_raw.lat);
        navi.position.longitude = utils::decode_lat_lon(gps_raw.lon);
        navi.position.altitude = utils::decode_altitude(gps_raw.alt);
        navi.course = utils::decode_cog_or_hdg(gps_raw.cog);
        navi.ground_speed = utils::decode_ground_speed(gps_raw.vel);
        navi.fix = gps_raw.fix_type as u8;
        navi.eph = gps_raw.eph;
        navi.epv = gps_raw.epv;
        navi.satellites_visible = gps_raw.satellites_visible;

        self.update_navigation(mav_id, navi).await;
    }

    pub async fn handle_sys_data(&mut self, mav_id: u8, sys_data: &SYS_STATUS_DATA) { 
        let mut system = self.mav_system_map.get(&mav_id).cloned().unwrap_or_default();

        system.battery_current = utils::decode_current(sys_data.current_battery);
        system.battery_voltage = utils::decode_voltage(sys_data.voltage_battery);
        system.battery_remaining = sys_data.battery_remaining;

        system.sensors.clear();

        let mut get_sensor_data = |sensor: SensorType, name: &str, mav_sensor: MavSysStatusSensor| {
            if sys_data.onboard_control_sensors_present.intersects(mav_sensor) {
                system.sensors.push(
                    Sensor {
                        name: name.to_owned(),
                        sensor,
                        enabled: sys_data.onboard_control_sensors_enabled.intersects(mav_sensor),
                        health: sys_data.onboard_control_sensors_health.intersects(mav_sensor)
                    }
                );
            }
        };

        get_sensor_data(SensorType::Ahrs, "AHRS", MavSysStatusSensor::MAV_SYS_STATUS_AHRS);

        get_sensor_data(SensorType::Accel,"Accel", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_ACCEL);
        get_sensor_data(SensorType::Gyro,"Gyro", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_GYRO);
        get_sensor_data(SensorType::Mag,"Mag", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_MAG);

        get_sensor_data(SensorType::Accel,"Accel 2", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_ACCEL2);
        get_sensor_data(SensorType::Gyro,"Gyro 2", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_GYRO2);
        get_sensor_data(SensorType::Mag,"Mag 2", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_MAG2);

        get_sensor_data(SensorType::Sns,"SNS (GPS)", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_GPS);
        get_sensor_data(SensorType::AbsPressure,"Abs. Pressure", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_ABSOLUTE_PRESSURE);
        get_sensor_data(SensorType::DiffPressure,"Diff. Pressure", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_DIFFERENTIAL_PRESSURE);
        get_sensor_data(SensorType::Laser,"Laser", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_LASER_POSITION);

        get_sensor_data(SensorType::Battery,"Battery", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_BATTERY);
        get_sensor_data(SensorType::Optical,"Optical", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_OPTICAL_FLOW);
        get_sensor_data(SensorType::Motors,"Motors", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_MOTOR_OUTPUTS);
        get_sensor_data(SensorType::RadioControl,"Radio Control", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_RC_RECEIVER);
        get_sensor_data(SensorType::SatComm,"Sat. Comm", MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_SATCOM);
        get_sensor_data(SensorType::Avoidance,"Avoidance", MavSysStatusSensor::MAV_SYS_STATUS_OBSTACLE_AVOIDANCE);

        system.arm_ready = sys_data.onboard_control_sensors_enabled.intersects(MavSysStatusSensor::MAV_SYS_STATUS_PREARM_CHECK);

        self.update_system(mav_id, system).await;
    }

    async fn update_flight(&mut self, mav_id: u8, flight: Flight) {
        self.mav_flight_map.insert(mav_id, flight.clone());

        let context = self.context.lock().await;
        if let Err(err) = context.send_telemetry(VehicleTelemetry{
            vehicle_id: context.vehicle_id_from_mav_id(&mav_id).unwrap(),
            timestamp: chrono::prelude::Utc::now().timestamp_millis(),
            flight: Some(flight),
            navigation: None,
            system: None
        }) {
            log::error!("Update flight telemetry error: {}", err);
        }
    }

    async fn update_navigation(&mut self, mav_id: u8, navigation: Navigation) {
        self.mav_navi_map.insert(mav_id, navigation.clone());

        let context = self.context.lock().await;
        if let Err(err) = context.send_telemetry(VehicleTelemetry{
            vehicle_id: context.vehicle_id_from_mav_id(&mav_id).unwrap(),
            timestamp: chrono::prelude::Utc::now().timestamp_millis(),
            flight: None,
            navigation: Some(navigation),
            system: None
        }) {
            log::error!("Update flight telemetry error: {}", err);
        }
    }

    async fn update_system(&mut self, mav_id: u8, system: System) {
        self.mav_system_map.insert(mav_id, system.clone());

        let context = self.context.lock().await;
        if let Err(err) = context.send_telemetry(VehicleTelemetry{
            vehicle_id: context.vehicle_id_from_mav_id(&mav_id).unwrap(),
            timestamp: chrono::prelude::Utc::now().timestamp_millis(),
            flight: None,
            navigation: None,
            system: Some(system)
        }) {
            log::error!("Update flight telemetry error: {}", err);
        }
    }
}