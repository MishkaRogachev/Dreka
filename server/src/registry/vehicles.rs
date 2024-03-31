
use std::sync::Arc;
use anyhow::Ok;
use surrealdb::{engine::local::Db, Surreal};

use crate::persistence::{repository, traits};
use crate::models::events::ServerEvent;
use crate::models::vehicles::{VehicleId, VehicleDescription, VehicleStatus, ProtocolId};

use super::bus;

#[derive(Clone)]
pub struct Persistence {
    vehicle_descriptions: Arc<dyn traits::IRepository<VehicleDescription> + Send + Sync>,
    vehicle_statuses: Arc<dyn traits::IRepository<VehicleStatus> + Send + Sync>,
    bus: bus::EventBus<ServerEvent>
}

impl Persistence {
    pub fn new(db: Surreal<Db>, bus: bus::EventBus<ServerEvent>) -> Self {
        Self {
            vehicle_descriptions: Arc::new(repository::Repository::new(db.clone(), "vehicle_descriptions")),
            vehicle_statuses: Arc::new(repository::Repository::new(db, "vehicle_statuses")),
            bus
        }
    }

    pub async fn save_vehicle(&self, vehicle: &VehicleDescription) -> anyhow::Result<VehicleDescription> {
        let vehicle = if vehicle.id.is_empty() {
            let same_protocol_exists = self.vehicle_by_protocol_id(&vehicle.protocol_id).await?;
            if same_protocol_exists.is_some() {
                return Err(anyhow::anyhow!("Vehicle with protocol_id {:?} already exists", vehicle.protocol_id));
            }

            let new_vehicle = self.vehicle_descriptions.create(vehicle).await?;
            self.vehicle_statuses.create(&VehicleStatus::default_for_id(&new_vehicle.id)).await?;
            new_vehicle
        } else {
            self.vehicle_descriptions.update(vehicle).await?
        };

        self.bus.publish(ServerEvent::VehicleUpdated { vehicle: vehicle.clone() })?;
        Ok(vehicle)
    }

    pub async fn delete_vehicle(&self, vehicle_id: &VehicleId) -> anyhow::Result<()> {
        self.vehicle_descriptions.delete(vehicle_id).await?;
        self.vehicle_statuses.delete(vehicle_id).await?;

        self.bus.publish(ServerEvent::VehicleRemoved { vehicle_id: vehicle_id.into() })?;
        Ok(())
    }

    pub async fn update_status(&self, status: &VehicleStatus) -> anyhow::Result<VehicleStatus> {
        let status = self.vehicle_statuses.update(status).await?;
        self.bus.publish(ServerEvent::VehicleStatusUpdated { status: status.clone() })?;
        Ok(status)
    }

    pub async fn vehicle(&self, vehicle_id: &VehicleId) -> anyhow::Result<VehicleDescription> {
        self.vehicle_descriptions.read(vehicle_id).await
    }

    pub async fn vehicle_by_protocol_id(&self, protocol_id: &ProtocolId) -> anyhow::Result<Option<VehicleDescription>> {
        let vehicles = self.vehicle_descriptions.read_where(
            "protocol_id", serde_json::json!(protocol_id)).await?;
        match vehicles.len() {
            0 => Ok(None),
            1 => Ok(Some(vehicles.first().cloned().unwrap())),
            _ => Err(anyhow::anyhow!("Multiple vehicles found for protocol_id: {:?}", protocol_id))
        }
    }

    pub async fn all_vehicles(&self) -> anyhow::Result<Vec<VehicleDescription>> {
        self.vehicle_descriptions.read_all().await
    }

    pub async fn status(&self, vehicle_id: &VehicleId) -> anyhow::Result<VehicleStatus> {
        self.vehicle_statuses.read(vehicle_id).await
    }

    pub async fn all_statuses(&self) -> anyhow::Result<Vec<VehicleStatus>> {
        self.vehicle_statuses.read_all().await
    }
}
