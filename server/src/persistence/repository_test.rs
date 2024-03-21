#[cfg(test)]
mod tests {
    use test_case::test_case;
    use surrealdb::{engine::local::Mem, Surreal};

    use crate::models::vehicles::{ProtocolId, VehicleDescription, VehicleFeatures, VehicleId, VehicleType};
    use crate::models::colors::EntityColor;
    use crate::persistence::{traits, repository};

    async fn setup() -> Box<dyn traits::IRepository<VehicleDescription>> {
        let db = Surreal::new::<Mem>(())
            .await
            .expect("Error establishing a database connection");
        db.use_ns("test").use_db("test").await.expect("Error setting namespace and database");

        Box::new(repository::Repository::new(db, "vehicle_descriptions"))
    }

    #[test_case(VehicleDescription {
        id: VehicleId::new(),
        name: "test_name_1".to_string(),
        color: EntityColor::Teal,
        vehicle_type: VehicleType::FixedWing,
        protocol_id: ProtocolId::MavlinkId{ mav_id: 1 },
        features: vec![VehicleFeatures::PetrolEngine, VehicleFeatures::Parachute]
    }; "vehicle 1")]

    #[test_case(VehicleDescription {
        id: "explicit_vehicle_id".to_string(),
        name: "test_name_2".to_string(),
        color: EntityColor::Cyan,
        vehicle_type: VehicleType::Vtol,
        protocol_id: ProtocolId::MavlinkId{ mav_id: 2 },
        features: vec![VehicleFeatures::Lidar]
    }; "vehicle 2")]

    #[tokio::test]
    async fn test_vehicles_crud(vehicle: VehicleDescription) {
        let repo = setup().await;

        // CREATE
        let vehicle = repo.create(&vehicle).await.expect("Error saving vehicle");
        assert_ne!(vehicle.id.len(), 0);

        // READ
        let vehicle_back = repo.read(&vehicle.id).await.expect("Error reading vehicle");
        assert_eq!(vehicle, vehicle_back);

        // UPDATE
        let vehicle = VehicleDescription{
            id: vehicle.id.clone(),
            name: "test_name_2".to_string(),
            color: EntityColor::Cyan,
            vehicle_type: VehicleType::Vtol,
            protocol_id: ProtocolId::MavlinkId{ mav_id: 2 },
            features: vec![VehicleFeatures::Lidar]
        };
        let vehicle_back = repo.update(&vehicle).await.expect("Error updating vehicle");
        assert_eq!(vehicle, vehicle_back);

        // DELETE
        repo.delete(&vehicle.id).await.expect("Error deleting vehicle");
    }

    #[test_case(vec![
        VehicleDescription {
            id: VehicleId::new(),
            name: "test_name_1".to_string(),
            color: EntityColor::Teal,
            vehicle_type: VehicleType::FixedWing,
            protocol_id: ProtocolId::MavlinkId{ mav_id: 1 },
            features: vec![VehicleFeatures::PetrolEngine, VehicleFeatures::Parachute]
        },
        VehicleDescription {
            id: "explicit_vehicle_id".to_string(),
            name: "test_name_2".to_string(),
            color: EntityColor::Cyan,
            vehicle_type: VehicleType::Vtol,
            protocol_id: ProtocolId::MavlinkId{ mav_id: 2 },
            features: vec![VehicleFeatures::Lidar]
        }
    ]; " list of vehicles")]

    #[tokio::test]
    async fn test_vehicles_select(vehices: Vec<VehicleDescription>) {
        let repo = setup().await;

        let mut updated_vehices = vec![];
        for vehicle in vehices.iter() {
            updated_vehices.push(repo.create(vehicle).await.expect("Error saving vehicle"));
        }
        assert_eq!(vehices.len(), updated_vehices.len());

        let vehices_back = repo.read_all().await.expect("Error reading vehicles");
        assert!(vehices_back.iter().all(|vehicle| updated_vehices.contains(&vehicle)));

        let vehicle_ids = updated_vehices.iter().map(|v| v.id.clone()).collect::<Vec<_>>();
        let vehicle_ids_back = repo.read_all_ids().await.expect("Error reading vehicle ids");
        assert!(vehicle_ids_back.iter().all(|vehicle_id| vehicle_ids.contains(&vehicle_id)));

        let vehicles_by_protocol_id = repo.read_where("protocol_id",
            serde_json::json!(updated_vehices[0].protocol_id))
            .await.expect("Error reading vehicles by protocol id");
        assert_eq!(vehicles_by_protocol_id.len(), 1);
        assert_eq!(vehicles_by_protocol_id[0], updated_vehices[0]);
    }
}
