use std::{sync::Arc, collections::HashMap};

use mavlink::{MavHeader, common::{MavMessage, ATTITUDE_DATA, VFR_HUD_DATA, GLOBAL_POSITION_INT_DATA, GPS_RAW_INT_DATA, SYS_STATUS_DATA, MavSysStatusSensor}};
use tokio::sync::Mutex;

use super::{context::MavlinkContext, utils};
use crate::models::telemetry::{FlightData, SnsData, SensorsData, SensorData, SensorType};

pub struct TelemetryHandler {
    context: Arc<Mutex<MavlinkContext>>,

    mav_flight_data: HashMap<u8, FlightData>,
    mav_sns_data: HashMap<u8, SnsData>,
    mav_sensors_data: HashMap<u8, SensorsData>,
}

impl TelemetryHandler {
    pub fn new(context: Arc<Mutex<MavlinkContext>>) -> Self {
        Self {
            context,
            mav_flight_data: HashMap::new(),
            mav_sns_data: HashMap::new(),
            mav_sensors_data: HashMap::new(),
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
        let mut flight_data = self.flight_data_for_mav(&mav_id);

        flight_data.pitch = utils::decode_angles(attitude.pitch);
        flight_data.roll = utils::decode_angles(attitude.roll);
        flight_data.yaw = utils::decode_angles(attitude.yaw);
        flight_data.timestamp = chrono::prelude::Utc::now().timestamp_millis();

        self.set_flight_data_for_mav(mav_id, flight_data).await;
    }

    pub async fn handle_vfr_hud(&mut self, mav_id: u8, vfr_hud: &VFR_HUD_DATA) {
        let mut flight_data = self.flight_data_for_mav(&mav_id);

        flight_data.indicated_airspeed = vfr_hud.airspeed;
        flight_data.true_airspeed = utils::to_true_airspeed(vfr_hud.airspeed, vfr_hud.alt);
        flight_data.ground_speed = vfr_hud.groundspeed;
        flight_data.climb = vfr_hud.climb;
        flight_data.altitude_amsl = vfr_hud.alt;
        flight_data.throttle = vfr_hud.throttle;
        flight_data.timestamp = chrono::prelude::Utc::now().timestamp_millis();

        self.set_flight_data_for_mav(mav_id, flight_data).await;
    }

    pub async fn handle_global_position(&mut self, mav_id: u8, global_pos: &GLOBAL_POSITION_INT_DATA) {
        let mut flight_data = self.flight_data_for_mav(&mav_id);

        flight_data.position.latitude = utils::decode_lat_lon(global_pos.lat);
        flight_data.position.longitude = utils::decode_lat_lon(global_pos.lon);
        flight_data.position.altitude = utils::decode_altitude(global_pos.alt);
        flight_data.position.frame = crate::models::spatial::GeodeticFrame::Wgs84AboveSeaLevel;
        flight_data.timestamp = chrono::prelude::Utc::now().timestamp_millis();

        self.set_flight_data_for_mav(mav_id, flight_data).await;
    }

    pub async fn handle_gps_raw(&mut self, mav_id: u8, gps_raw: &GPS_RAW_INT_DATA) { 
        let mut sns_data = self.sns_data_for_mav(&mav_id);

        sns_data.position.latitude = utils::decode_lat_lon(gps_raw.lat);
        sns_data.position.longitude = utils::decode_lat_lon(gps_raw.lon);
        sns_data.position.altitude = utils::decode_altitude(gps_raw.alt);
        sns_data.course = utils::decode_cog_or_hdg(gps_raw.cog);
        sns_data.ground_speed = utils::decode_ground_speed(gps_raw.vel);
        sns_data.fix = gps_raw.fix_type as u8;
        sns_data.eph = gps_raw.eph;
        sns_data.epv = gps_raw.epv;
        sns_data.satellites_visible = gps_raw.satellites_visible;
        sns_data.timestamp = chrono::prelude::Utc::now().timestamp_millis();

        self.set_sns_data_for_mav(mav_id, sns_data).await;
    }

    pub async fn handle_sys_data(&mut self, mav_id: u8, sys_data: &SYS_STATUS_DATA) { 
        let mut sensors_data = self.sensors_data_for_mav(&mav_id);

        sensors_data.battery_current = utils::decode_current(sys_data.current_battery);
        sensors_data.battery_voltage = utils::decode_voltage(sys_data.voltage_battery);
        sensors_data.battery_remaining = sys_data.battery_remaining;

        sensors_data.sensors.clear();

        let mut get_sensor_data = |sensor: SensorType, name: &str, mav_sensor: MavSysStatusSensor| {
            if sys_data.onboard_control_sensors_present.intersects(mav_sensor) {
                sensors_data.sensors.push(
                    SensorData {
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

        sensors_data.arm_ready = sys_data.onboard_control_sensors_enabled.intersects(MavSysStatusSensor::MAV_SYS_STATUS_PREARM_CHECK);

        sensors_data.timestamp = chrono::prelude::Utc::now().timestamp_millis();

        self.set_sensors_data_for_mav(mav_id, sensors_data).await;
    }

    fn flight_data_for_mav(&self, mav_id: &u8) -> FlightData {
        match self.mav_flight_data.get(mav_id) {
            Some(flight_data) => flight_data.to_owned(),
            None => FlightData::default(),
        }
    }

    async fn set_flight_data_for_mav(&mut self, mav_id: u8, mut flight_data: FlightData) {
        let context = self.context.lock().await;

        if let Some(vehicle_id) = context.vehicle_id_from_mav_id(&mav_id) {
            flight_data.id = vehicle_id;

            // let saved = context.repository.upsert("vehicle_flight_data", &flight_data).await;
            // if let Err(err) = saved {
            //     log::error!("Save vehicle flight data error: {:?}", &err);
            // }
        }

        self.mav_flight_data.insert(mav_id, flight_data);
    }

    fn sns_data_for_mav(&self, mav_id: &u8) -> SnsData {
        match self.mav_sns_data.get(mav_id) {
            Some(sns_data) => sns_data.to_owned(),
            None => SnsData::default(),
        }
    }

    async fn set_sns_data_for_mav(&mut self, mav_id: u8, mut sns_data: SnsData) {
        let context = self.context.lock().await;

        if let Some(vehicle_id) = context.vehicle_id_from_mav_id(&mav_id) {
            sns_data.id = vehicle_id;

            // let saved = context.repository.upsert("vehicle_sns_data", &sns_data).await;
            // if let Err(err) = saved {
            //     log::error!("Save vehicle sns data error: {:?}", &err);
            // }
        }

        self.mav_sns_data.insert(mav_id, sns_data);
    }

    fn sensors_data_for_mav(&self, mav_id: &u8) -> SensorsData {
        match self.mav_sensors_data.get(mav_id) {
            Some(sns_data) => sns_data.to_owned(),
            None => SensorsData::default(),
        }
    }

    async fn set_sensors_data_for_mav(&mut self, mav_id: u8, mut sensors_data: SensorsData) {
        let context = self.context.lock().await;

        if let Some(vehicle_id) = context.vehicle_id_from_mav_id(&mav_id) {
            sensors_data.id = vehicle_id;

            // let saved = context.repository.upsert("vehicle_sensors_data", &sensors_data).await;
            // if let Err(err) = saved {
            //     log::error!("Save vehicle sns data error: {:?}", &err);
            // }
        }

        self.mav_sensors_data.insert(mav_id, sensors_data);
    }
}