use mavlink::common::*;

use crate::models::{telemetry::*, spatial::*};
use super::{handler, protocol::telemetry as protocol};

impl handler::Handler {
    pub async fn handle_attitude(&mut self, mav_id: u8, attitude: &ATTITUDE_DATA) {
        let (mut flight, vehicle_id) = match self.vehicle_id_from_mav_id(&mav_id) {
            Some(vehicle_id) => (self.dal.telemetry_flight(&vehicle_id).await.unwrap_or(
                Flight::default_for_id(&vehicle_id)), vehicle_id),
            None => return
        };

        flight.pitch = protocol::decode_angles(attitude.pitch);
        flight.roll = protocol::decode_angles(attitude.roll);
        flight.yaw = protocol::decode_angles(attitude.yaw);

        if let Err(err) = self.dal.save_telemetry_flight(vehicle_id, flight).await {
            log::error!("Save flight telemetry error: {}", err);
        }
    }

    pub async fn handle_vfr_hud(&mut self, mav_id: u8, vfr_hud: &VFR_HUD_DATA) {
        let (mut flight, vehicle_id) = match self.vehicle_id_from_mav_id(&mav_id) {
            Some(vehicle_id) => (self.dal.telemetry_flight(&vehicle_id).await.unwrap_or(
                Flight::default_for_id(&vehicle_id)), vehicle_id),
            None => return
        };

        flight.indicated_airspeed = vfr_hud.airspeed;
        flight.true_airspeed = protocol::to_true_airspeed(vfr_hud.airspeed, vfr_hud.alt);
        flight.ground_speed = vfr_hud.groundspeed;
        flight.climb = vfr_hud.climb;
        flight.altitude_amsl = vfr_hud.alt;
        flight.throttle = vfr_hud.throttle;

        if let Err(err) = self.dal.save_telemetry_flight(vehicle_id, flight).await {
            log::error!("Save flight telemetry error: {}", err);
        }
    }

    pub async fn handle_global_position(&mut self, mav_id: u8, global_pos: &GLOBAL_POSITION_INT_DATA) {
        let (mut navigation, vehicle_id) = match self.vehicle_id_from_mav_id(&mav_id) {
            Some(vehicle_id) => (self.dal.telemetry_navigation(&vehicle_id).await.unwrap_or(
                Navigation::default_for_id(&vehicle_id)), vehicle_id),
            None => return
        };

        navigation.position.latitude = protocol::decode_lat_lon(global_pos.lat);
        navigation.position.longitude = protocol::decode_lat_lon(global_pos.lon);
        navigation.position.altitude = protocol::decode_altitude(global_pos.alt);
        navigation.position.frame = GeodeticFrame::Wgs84AboveSeaLevel;

        if let Err(err) = self.dal.save_telemetry_navigation(vehicle_id, navigation).await {
            log::error!("Save navigation telemetry error: {}", err);
        }
    }

    pub async fn handle_home_position(&mut self, mav_id: u8, home_pos: &HOME_POSITION_DATA) {
        let (mut navigation, vehicle_id) = match self.vehicle_id_from_mav_id(&mav_id) {
            Some(vehicle_id) => (self.dal.telemetry_navigation(&vehicle_id).await.unwrap_or(
                Navigation::default_for_id(&vehicle_id)), vehicle_id),
            None => return
        };

        navigation.home_position.latitude = protocol::decode_lat_lon(home_pos.latitude);
        navigation.home_position.longitude = protocol::decode_lat_lon(home_pos.longitude);
        navigation.home_position.altitude = protocol::decode_altitude(home_pos.altitude);
        navigation.home_position.frame = GeodeticFrame::Wgs84AboveSeaLevel;

        if let Err(err) = self.dal.save_telemetry_navigation(vehicle_id, navigation).await {
            log::error!("Save navigation telemetry error: {}", err);
        }
    }

    pub async fn handle_gps_raw(&mut self, mav_id: u8, gps_raw: &GPS_RAW_INT_DATA) { 
        let (mut raw_sns, vehicle_id) = match self.vehicle_id_from_mav_id(&mav_id) {
            Some(vehicle_id) => (self.dal.telemetry_raw_sns(&vehicle_id).await.unwrap_or(
                RawSns::default_for_id(&vehicle_id)), vehicle_id),
            None => return
        };

        raw_sns.position.latitude = protocol::decode_lat_lon(gps_raw.lat);
        raw_sns.position.longitude = protocol::decode_lat_lon(gps_raw.lon);
        raw_sns.position.altitude = protocol::decode_altitude(gps_raw.alt);
        raw_sns.course = protocol::decode_cog_or_hdg(gps_raw.cog);
        raw_sns.ground_speed = protocol::decode_ground_speed(gps_raw.vel);
        raw_sns.fix = gps_raw.fix_type as u8;
        raw_sns.eph = gps_raw.eph;
        raw_sns.epv = gps_raw.epv;
        raw_sns.satellites_visible = gps_raw.satellites_visible;

        if let Err(err) = self.dal.save_telemtry_raw_sns(vehicle_id, raw_sns).await {
            log::error!("Save raw_sns telemetry error: {}", err);
        }
    }

    pub async fn handle_sys_data(&mut self, mav_id: u8, sys_data: &SYS_STATUS_DATA) { 
        let (mut system, vehicle_id) = match self.vehicle_id_from_mav_id(&mav_id) {
            Some(vehicle_id) => (self.dal.telemetry_system(&vehicle_id).await.unwrap_or(
                System::default_for_id(&vehicle_id)), vehicle_id),
            None => return
        };

        system.battery_current = protocol::decode_current(sys_data.current_battery);
        system.battery_voltage = protocol::decode_voltage(sys_data.voltage_battery);
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

        if let Err(err) = self.dal.save_telemetry_system(vehicle_id, system).await {
            log::error!("Save system telemetry error: {}", err);
        }
    }

    pub async fn handle_mission_item_current(&mut self, mav_id: u8, data: &MISSION_CURRENT_DATA) {
        let mut status = match self.mission_id_from_mav_id(&mav_id).await {
            Some(mission_id) => self.dal.mission_status(&mission_id).await.unwrap(),
            None => return
        };

        // TODO: mavlink 2 data.mission_state

        // -1 shift for home item
        status.progress.current = if data.seq > 0 { data.seq - 1 } else { 0 };
        log::info!("Mission item {} reached by MAVLink {}", data.seq, mav_id);

        if let Err(err) = self.dal.update_mission_status(status.clone()).await {
            log::error!("Error updating mission status: {}", err);
        }
    }

    pub async fn handle_mission_item_reached(&mut self, mav_id: u8, data: &MISSION_ITEM_REACHED_DATA) {
        let mut status = match self.mission_id_from_mav_id(&mav_id).await {
            Some(mission_id) => self.dal.mission_status(&mission_id).await.unwrap(),
            None => return
        };

        // -1 shift for home item
        status.progress.reached.push(data.seq - 1);

        if let Err(err) = self.dal.update_mission_status(status.clone()).await {
            log::error!("Error updating mission status: {}", err);
        }
    }
}
