use super::dal::Dal;

use crate::models::events::ServerEvent;
use crate::models::vehicles::*;

const TB_VEHICLE_DESCRIPTIONS: &str = "vehicle_descriptions";
const TB_VEHICLE_STATUSES: &str = "vehicle_statuses";

impl Dal {
    pub async fn save_vehicle(&self, vehicle: VehicleDescription) -> anyhow::Result<VehicleDescription> {
        let vehicle = if vehicle.id.is_empty() {
            let same_protocol_exists = self.vehicle_by_protocol_id(&vehicle.protocol_id).await?;
            if same_protocol_exists.is_some() {
                return Err(anyhow::anyhow!("Vehicle with protocol_id {:?} already exists", vehicle.protocol_id));
            }

            let new_vehicle = self.dao.create(TB_VEHICLE_DESCRIPTIONS, vehicle).await?;
            self.dao.create(TB_VEHICLE_STATUSES, VehicleStatus::default_for_id(&new_vehicle.id)).await?;
            self.create_new_mission(&new_vehicle.id).await?;
            new_vehicle
        } else {
            self.dao.update(TB_VEHICLE_DESCRIPTIONS, vehicle).await?
        };

        self.bus.publish(ServerEvent::VehicleUpserted { vehicle: vehicle.clone() })?;
        Ok(vehicle)
    }

    pub async fn delete_vehicle(&self, vehicle_id: &VehicleId) -> anyhow::Result<()> {
        let mission_for_vehicle = self.mission_assignment_by_vehicle_id(&vehicle_id).await?;
        if let Some(mission_for_vehicle) = mission_for_vehicle {
            self.delete_mission(&mission_for_vehicle.id).await?
        }

        self.dao.delete(TB_VEHICLE_STATUSES, vehicle_id).await?;
        self.dao.delete(TB_VEHICLE_DESCRIPTIONS, vehicle_id).await?;

        self.bus.publish(ServerEvent::VehicleRemoved { vehicle_id: vehicle_id.into() })?;
        Ok(())
    }

    pub async fn update_vehicle_status(&self, status: VehicleStatus) -> anyhow::Result<VehicleStatus> {
        let status = self.dao.update(TB_VEHICLE_STATUSES, status).await?;
        self.bus.publish(ServerEvent::VehicleStatusUpdated { status: status.clone() })?;
        Ok(status)
    }

    pub async fn vehicle(&self, vehicle_id: &VehicleId) -> anyhow::Result<VehicleDescription> {
        self.dao.select_one(TB_VEHICLE_DESCRIPTIONS, vehicle_id).await
    }

    pub async fn vehicle_by_protocol_id(&self, protocol_id: &ProtocolId) -> anyhow::Result<Option<VehicleDescription>> {
        let vehicles = self.dao.select_where(TB_VEHICLE_DESCRIPTIONS, "protocol_id", protocol_id).await?;
        match vehicles.len() {
            0 => Ok(None),
            1 => Ok(Some(vehicles.first().cloned().unwrap())),
            _ => Err(anyhow::anyhow!("Multiple vehicles found for protocol_id: {:?}", protocol_id))
        }
    }

    pub async fn all_vehicles(&self) -> anyhow::Result<Vec<VehicleDescription>> {
        self.dao.select_all(TB_VEHICLE_DESCRIPTIONS).await
    }

    pub async fn vehcile_status(&self, vehicle_id: &VehicleId) -> anyhow::Result<VehicleStatus> {
        self.dao.select_one(TB_VEHICLE_STATUSES, vehicle_id).await
    }

    pub async fn all_vehicles_statuses(&self) -> anyhow::Result<Vec<VehicleStatus>> {
        self.dao.select_all(TB_VEHICLE_STATUSES).await
    }
}
