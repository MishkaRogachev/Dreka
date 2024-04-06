use std::sync::Arc;
use surrealdb::{engine::local::Db, Surreal};

use crate::persistence::{repository, traits};
use crate::models::vehicles::VehicleId;
use crate::models::events::ServerEvent;
use crate::models::mission::*;

use super::bus;

#[derive(Clone)]
pub struct Persistence {
    mission_routes: Arc<dyn traits::IRepository<MissionRoute> + Send + Sync>,
    mission_statuses: Arc<dyn traits::IRepository<MissionStatus> + Send + Sync>,
    vehicle_missions: Arc<dyn traits::IRepository<VehicleMission> + Send + Sync>,
    bus: bus::EventBus<ServerEvent>
}

impl Persistence {
    pub fn new(db: Surreal<Db>, bus: bus::EventBus<ServerEvent>) -> Self {
        Self {
            mission_routes: Arc::new(repository::Repository::new(db.clone(), "mission_routes")),
            mission_statuses: Arc::new(repository::Repository::new(db.clone(), "mission_update_statuses")),
            vehicle_missions: Arc::new(repository::Repository::new(db, "vehicle_missions")),
            bus
        }
    }

    pub async fn save_new_mission(&self, mission: &Mission) -> anyhow::Result<Mission> {
        if !mission.vehicle_id.is_empty() {
            let vehicle_mission_exists = self.vehicle_mission(&mission.vehicle_id).await?;
            if vehicle_mission_exists.is_some() {
                return Err(anyhow::anyhow!("Mission for vehicle_id {:?} already exists", mission.vehicle_id));
            }
        }

        // Crete new vehicle-mission mapping
        let vehicle_mission = self.vehicle_missions.create(&VehicleMission{
            id: String::new(), // will be generated
            vehicle_id: mission.vehicle_id.clone(),
        }).await?;

        // Create new mission route
        let route = self.mission_routes.create(&MissionRoute{
            id: vehicle_mission.id.clone(),
            items: mission.route.items.clone()
        }).await?;

        // Create new mission status
        let status = self.mission_statuses.create(&&MissionStatus{
            id: vehicle_mission.id.clone(),
            state: mission.status.state.clone(),
            progress: mission.status.progress.clone(),
        }
        ).await?;

        let saved_mission = Mission {
            id: vehicle_mission.id,
            vehicle_id: vehicle_mission.vehicle_id,
            route,
            status
        };

        self.bus.publish(ServerEvent::MissionUpdated { mission: saved_mission.clone() })?;
        Ok(saved_mission)
    }

    pub async fn delete_mission(&self, mission_id: &MissionId) -> anyhow::Result<()> {
        self.vehicle_missions.delete(mission_id).await?;
        self.mission_routes.delete(mission_id).await?;
        self.mission_statuses.delete(mission_id).await?;

        self.bus.publish(ServerEvent::MissionRemoved { mission_id: mission_id.into() })?;
        Ok(())
    }

    pub async fn update_route(&self, route: &MissionRoute) -> anyhow::Result<MissionRoute> {
        if route.id.is_empty() {
            return Err(anyhow::anyhow!("MissionRoute id is empty"));
        }

        let route = self.mission_routes.update(route).await?;
        self.bus.publish(ServerEvent::MissionRouteUpdated { route: route.clone() })?;
        Ok(route)
    }

    pub async fn update_status(&self, status: &MissionStatus) -> anyhow::Result<MissionStatus> {
        if status.id.is_empty() {
            return Err(anyhow::anyhow!("MissionStatus id is empty"));
        }

        let status = self.mission_statuses.update(status).await?;
        self.bus.publish(ServerEvent::MissionStatusUpdated { status: status.clone() })?;
        Ok(status)
    }

    pub async fn mission(&self, mission_id: &MissionId) -> anyhow::Result<Mission> {
        let vehicle_mission = self.vehicle_missions.read(mission_id).await?;
        let route = self.mission_routes.read(mission_id).await?;
        let status = self.mission_statuses.read(mission_id).await?;
        Ok(Mission {
            id: vehicle_mission.id,
            vehicle_id: vehicle_mission.vehicle_id,
            route,
            status
        })
    }

    pub async fn all_missions(&self) -> anyhow::Result<Vec<Mission>> {
        let mut missions = Vec::new();
        for vehicle_mission in self.vehicle_missions.read_all().await? {
            let route = self.mission_routes.read(&vehicle_mission.id).await?;
            let status = self.mission_statuses.read(&vehicle_mission.id).await?;
            let mission = Mission {
                id: vehicle_mission.id,
                vehicle_id: vehicle_mission.vehicle_id,
                route,
                status
            };
            missions.push(mission);
        }
        Ok(missions)
    }

    pub async fn vehicle_mission(&self, vehicle_id: &VehicleId) -> anyhow::Result<Option<VehicleMission>> {
        let vehicle_missions = self.vehicle_missions.read_where(
            "vehicle_id", serde_json::json!(vehicle_id)).await?;
        match vehicle_missions.len() {
            0 => Ok(None),
            1 => Ok(Some(vehicle_missions.first().cloned().unwrap())),
            _ => Err(anyhow::anyhow!("Multiple missions found for vehicle_id: {:?}", vehicle_id))
        }
    }
}
