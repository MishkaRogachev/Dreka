
use std::sync::Arc;
use surrealdb::{engine::local::Db, Surreal};

use crate::persistence::{repository, traits};
use crate::models::vehicles::{VehicleId, VehicleDescription, VehicleStatus};

#[derive(Clone)]
pub struct Persistence {
    vehicle_descriptions: Arc<dyn traits::IRepository<VehicleDescription> + Send + Sync>,
    vehicle_statuses: Arc<dyn traits::IRepository<VehicleStatus> + Send + Sync>,
}

impl Persistence {
    pub fn new(db: Surreal<Db>) -> Self {
        Self {
            vehicle_descriptions: Arc::new(repository::Repository::new(db.clone(), "vehicle_descriptions")),
            vehicle_statuses: Arc::new(repository::Repository::new(db, "vehicle_statuses"))
        }
    }

    pub async fn save_vehicle(&self, vehicle: &VehicleDescription) -> anyhow::Result<VehicleDescription> {
        if vehicle.id.is_empty() {
            let vehicle = self.vehicle_descriptions.create(vehicle).await?;
            self.vehicle_statuses.create(&VehicleStatus::default_for_id(&vehicle.id)).await?;
            Ok(vehicle)

        } else {
            let vehicle = self.vehicle_descriptions.update(vehicle).await?;
            Ok(vehicle)
        }
    }

    pub async fn delete_vehicle(&self, vehicle_id: &VehicleId) -> anyhow::Result<()> {
        self.vehicle_descriptions.delete(vehicle_id).await?;
        self.vehicle_statuses.delete(vehicle_id).await?;
        Ok(())
    }

    pub async fn update_status(&self, status: &VehicleStatus) -> anyhow::Result<VehicleStatus> {
        self.vehicle_statuses.update(status).await
    }

    pub async fn vehicle(&self, vehicle_id: &VehicleId) -> anyhow::Result<VehicleDescription> {
        self.vehicle_descriptions.read(vehicle_id).await
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
