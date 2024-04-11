use super::dal::Dal;

use crate::models::events::ServerEvent;

use crate::models::vehicles::VehicleId;
use crate::models::missions::*;

const TB_MISSION_ASSIGNMENTS: &str = "mission_assignments";
const TB_MISSION_ROUTES: &str = "mission_routes";
const TB_MISSION_STATUSES: &str = "mission_statuses";

impl Dal {
    pub async fn create_new_mission(&self, vehicle_id: &VehicleId) -> anyhow::Result<Mission> {
        if !vehicle_id.is_empty() {
            let vehicle_mission_exists = self.mission_assignment_by_vehicle_id(&vehicle_id).await?;
            if vehicle_mission_exists.is_some() {
                return Err(anyhow::anyhow!("Mission for vehicle_id {} already exists", vehicle_id));
            }
        }

        // Crete new vehicle-mission mapping
        let assignment = self.dao.create(TB_MISSION_ASSIGNMENTS, MissionAssignment{
            id: String::new(), // will be generated
            vehicle_id: vehicle_id.clone(),
        }).await?;

        // Create new mission route
        let route = self.dao.create(TB_MISSION_ROUTES, MissionRoute{
            id: assignment.id.clone(),
            items: Vec::new(),
        }).await?;

        // Create new mission status
        let status = self.dao.create(TB_MISSION_STATUSES, MissionStatus{
            id: assignment.id.clone(),
            state: MissionUpdateState::NotActual {},
            progress: MissionProgress::OnHold {},
        }
        ).await?;

        let saved_mission = Mission {
            id: assignment.id,
            vehicle_id: assignment.vehicle_id,
            route,
            status
        };

        self.bus.publish(ServerEvent::MissionUpserted { mission: saved_mission.clone() })?;
        Ok(saved_mission)
    }

    pub async fn delete_mission(&self, mission_id: &MissionId) -> anyhow::Result<()> {
        self.dao.delete(TB_MISSION_STATUSES, mission_id).await?;
        self.dao.delete(TB_MISSION_ROUTES, mission_id).await?;
        self.dao.delete(TB_MISSION_ASSIGNMENTS, mission_id).await?;

        self.bus.publish(ServerEvent::MissionRemoved { mission_id: mission_id.into() })?;
        Ok(())
    }

    pub async fn update_mission(&self, mission: Mission) -> anyhow::Result<Mission> {
        let route = self.dao.update(TB_MISSION_ROUTES, mission.route).await?;
        let status = self.dao.update(TB_MISSION_STATUSES, mission.status).await?;

        let updated_mission = Mission {
            id: mission.id.clone(),
            vehicle_id: mission.vehicle_id.clone(),
            route,
            status
        };

        self.bus.publish(ServerEvent::MissionUpserted { mission: updated_mission.clone() })?;
        Ok(updated_mission)
    }

    pub async fn update_route(&self, route: MissionRoute) -> anyhow::Result<MissionRoute> {
        if route.id.is_empty() {
            return Err(anyhow::anyhow!("MissionRoute id is empty"));
        }

        let route = self.dao.update(TB_MISSION_ROUTES, route).await?;
        self.bus.publish(ServerEvent::MissionRouteUpdated { route: route.clone() })?;
        Ok(route)
    }

    pub async fn upsert_route_item(&self, mission_id: &MissionId, item: MissionRouteItem, index: u16) -> anyhow::Result<Vec<(u16, MissionRouteItem)>> {
        let mut route: MissionRoute = self.dao.select_one(TB_MISSION_ROUTES, mission_id).await?;
        let index = index as usize;

        let mut new_items = Vec::new();

        if index < route.items.len() {
            route.items[index] = item.clone();
        } else {
            while index > route.items.len() {
                let gap = MissionRouteItem::Gap {};
                new_items.push((route.items.len() as u16, gap.clone()));
                route.items.push(gap);
            }
            route.items.push(item.clone());
        }

        new_items.push((index as u16, item));
        for (index, item) in new_items.iter() {
            self.bus.publish(ServerEvent::MissionRouteItemUpserted {
                mission_id: mission_id.clone(),
                index: index.clone(),
                item: item.clone()
            })?;
        }
        self.dao.update(TB_MISSION_ROUTES, route).await?;
        Ok(new_items)
    }

    pub async fn remove_route_item(&self, mission_id: &MissionId, index: u16) -> anyhow::Result<u16> {
        let mut route: MissionRoute = self.dao.select_one(TB_MISSION_ROUTES, mission_id).await?;

        route.items.remove(index as usize);
        self.dao.update(TB_MISSION_ROUTES, route).await?;
        self.bus.publish(ServerEvent::MissionRouteItemRemoved { mission_id: mission_id.clone(), index })?;
        Ok(index)
    }

    pub async fn update_mission_status(&self, status: MissionStatus) -> anyhow::Result<MissionStatus> {
        if status.id.is_empty() {
            return Err(anyhow::anyhow!("MissionStatus id is empty"));
        }

        let status = self.dao.update(TB_MISSION_STATUSES, status).await?;
        self.bus.publish(ServerEvent::MissionStatusUpdated { status: status.clone() })?;
        Ok(status)
    }

    pub async fn mission(&self, mission_id: &MissionId) -> anyhow::Result<Mission> {
        let assignment: MissionAssignment = self.dao.select_one(TB_MISSION_ASSIGNMENTS, mission_id).await?;
        let route = self.dao.select_one(TB_MISSION_ROUTES, mission_id).await?;
        let status = self.dao.select_one(TB_MISSION_STATUSES, mission_id).await?;
        Ok(Mission {
            id: assignment.id,
            vehicle_id: assignment.vehicle_id,
            route,
            status
        })
    }

    pub async fn all_missions(&self) -> anyhow::Result<Vec<Mission>> {
        let mut missions = Vec::new();
        for assignment in self.dao.select_all::<MissionAssignment>(TB_MISSION_ASSIGNMENTS).await? {
            let route = self.dao.select_one(TB_MISSION_ROUTES, &assignment.id).await?;
            let status = self.dao.select_one(TB_MISSION_STATUSES, &assignment.id).await?;
            let mission = Mission {
                id: assignment.id,
                vehicle_id: assignment.vehicle_id,
                route,
                status
            };
            missions.push(mission);
        }
        Ok(missions)
    }

    pub async fn mission_assignment(&self, id: &MissionId) -> anyhow::Result<MissionAssignment> {
        self.dao.select_one(TB_MISSION_ASSIGNMENTS, id).await
    }

    pub async fn mission_assignment_by_vehicle_id(&self, vehicle_id: &VehicleId) -> anyhow::Result<Option<MissionAssignment>> {
        let assignments = self.dao.select_where(
            TB_MISSION_ASSIGNMENTS, "vehicle_id", vehicle_id).await?;
        match assignments.len() {
            0 => Ok(None),
            1 => Ok(Some(assignments.first().cloned().unwrap())),
            _ => Err(anyhow::anyhow!("Multiple missions found for vehicle_id: {:?}", vehicle_id))
        }
    }

    pub async fn mission_route(&self, mission_id: &MissionId) -> anyhow::Result<MissionRoute> {
        self.dao.select_one(TB_MISSION_ROUTES, mission_id).await
    }

    pub async fn mission_status(&self, mission_id: &MissionId) -> anyhow::Result<MissionStatus> {
        self.dao.select_one(TB_MISSION_STATUSES, mission_id).await
    }
}
