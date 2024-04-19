use super::dal::Dal;

use crate::models::{events::ServerEvent, telemetry::*, vehicles::VehicleId};

const TB_TELEMETRY_FLIGHT: &str = "telemetry_flight";
const TB_TELEMETRY_NAVIGATION: &str = "telemetry_navigation";
const TB_TELEMETRY_RAW_SNS: &str = "telemetry_raw_sns";
const TB_TELEMETRY_SYSTEM: &str = "telemetry_system";

impl Dal {
    pub async fn save_telemetry_flight(&self, vehicle_id: VehicleId, mut flight: Flight) -> anyhow::Result<Flight> {
        flight.timestamp = chrono::Utc::now().timestamp();
        let flight = if flight.id.is_empty() {
            self.dao.create(TB_TELEMETRY_FLIGHT, flight).await?
        } else {
            self.dao.update(TB_TELEMETRY_FLIGHT, flight).await?
        };
        self.bus.publish(ServerEvent::FlightUpdated { vehicle_id, flight: flight.clone() })?;
        Ok(flight)
    }

    pub async fn save_telemetry_navigation(&self, vehicle_id: VehicleId, mut navigation: Navigation) -> anyhow::Result<Navigation> {
        navigation.timestamp = chrono::Utc::now().timestamp();
        let navigation = if navigation.id.is_empty() {
            self.dao.create(TB_TELEMETRY_NAVIGATION, navigation).await?
        } else {
            self.dao.update(TB_TELEMETRY_NAVIGATION, navigation).await?
        };
        self.bus.publish(ServerEvent::NavigationUpdated { vehicle_id, navigation: navigation.clone() })?;
        Ok(navigation)
    }

    pub async fn save_telemtry_raw_sns(&self, vehicle_id: VehicleId, mut raw_sns: RawSns) -> anyhow::Result<RawSns> {
        raw_sns.timestamp = chrono::Utc::now().timestamp();
        let raw_sns = if raw_sns.id.is_empty() {
            self.dao.create(TB_TELEMETRY_RAW_SNS, raw_sns).await?
        } else {
            self.dao.update(TB_TELEMETRY_RAW_SNS, raw_sns).await?
        };
        self.bus.publish(ServerEvent::RawSnsUpdated { vehicle_id, raw_sns: raw_sns.clone() })?;
        Ok(raw_sns)
    }

    pub async fn save_telemetry_system(&self, vehicle_id: VehicleId, mut system: System) -> anyhow::Result<System> {
        system.timestamp = chrono::Utc::now().timestamp();
        let system = if system.id.is_empty() {
            self.dao.create(TB_TELEMETRY_SYSTEM, system).await?
        } else {
            self.dao.update(TB_TELEMETRY_SYSTEM, system).await?
        };
        self.bus.publish(ServerEvent::SystemUpdated { vehicle_id, system: system.clone() })?;
        Ok(system)
    }

    pub async fn telemetry_flight(&self, vehicle_id: &VehicleId) -> anyhow::Result<Flight> {
        self.dao.select_one(TB_TELEMETRY_FLIGHT, vehicle_id).await
    }

    pub async fn telemetry_navigation(&self, vehicle_id: &VehicleId) -> anyhow::Result<Navigation> {
        self.dao.select_one(TB_TELEMETRY_NAVIGATION, vehicle_id).await
    }

    pub async fn telemetry_raw_sns(&self, vehicle_id: &VehicleId) -> anyhow::Result<RawSns> {
        self.dao.select_one(TB_TELEMETRY_RAW_SNS, vehicle_id).await
    }

    pub async fn telemetry_system(&self, vehicle_id: &VehicleId) -> anyhow::Result<System> {
        self.dao.select_one(TB_TELEMETRY_SYSTEM, vehicle_id).await
    }
}
