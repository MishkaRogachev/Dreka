use std::{sync::Arc, collections::HashMap};

use mavlink::{MavHeader, common::{MavMessage, ATTITUDE_DATA, VFR_HUD_DATA, GLOBAL_POSITION_INT_DATA, GPS_RAW_INT_DATA}};
use tokio::sync::Mutex;

use super::{context::MavlinkContext, utils};
use crate::models::telemetry::{FlightData, SnsData};

pub struct TelemetryHandler {
    context: Arc<Mutex<MavlinkContext>>,

    mav_flight_data: HashMap<u8, FlightData>,
    mav_sns_data: HashMap<u8, SnsData>,
}

impl TelemetryHandler {
    pub fn new(context: Arc<Mutex<MavlinkContext>>) -> Self {
        Self {
            context,
            mav_flight_data: HashMap::new(),
            mav_sns_data: HashMap::new(),
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

        self.set_sns_data_for_mav(mav_id, sns_data).await;
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

            let saved = context.repository.upsert("vehicle_flight_data", &flight_data).await;
            if let Err(err) = saved {
                println!("Save vehicle flight data error: {:?}", &err);
            }
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

            let saved = context.repository.upsert("vehicle_sns_data", &sns_data).await;
            if let Err(err) = saved {
                println!("Save vehicle sns data error: {:?}", &err);
            }
        }

        self.mav_sns_data.insert(mav_id, sns_data);
    }
}