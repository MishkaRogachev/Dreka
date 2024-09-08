
use surrealdb::{engine::local::Mem, Surreal};
use test_case::test_case;

use crate::models::{colors::EntityColor, vehicles::*};

#[test_case(VehicleDescription {
    id: VehicleId::new(),
    name: "test_name_1".to_string(),
    color: EntityColor::Teal,
    vehicle_type: VehicleType::FixedWing,
    protocol_id: ProtocolId::MavlinkId{ mav_id: 1 },
    features: vec![VehicleFeatures::PetrolEngine, VehicleFeatures::Parachute],
    available_modes: vec![VehicleMode::RTL, VehicleMode::Loiter]
}; "vehicle 1")]

#[test_case(VehicleDescription {
    id: "explicit_vehicle_id".to_string(),
    name: "test_name_2".to_string(),
    color: EntityColor::Cyan,
    vehicle_type: VehicleType::Vtol,
    protocol_id: ProtocolId::MavlinkId{ mav_id: 2 },
    features: vec![VehicleFeatures::Lidar],
    available_modes: Vec::new()
}; "vehicle 2")]

#[tokio::test]
async fn test_vehicles_dao_operations(vehicle: VehicleDescription) {
    let db = Surreal::new::<Mem>(()).await
        .expect("Error establishing a database connection");
    db.use_ns("test").use_db("test").await
        .expect("Error setting namespace and database");

    let dao = super::surreal_dao::Dao::new(db);

    // CREATE
    let vehicle = dao.create("vehicles", vehicle).await
        .expect("Error saving vehicle");
    assert_ne!(vehicle.id.len(), 0);

    // SELECT
    let vehicle_back = dao.select_one::<VehicleDescription>("vehicles", &vehicle.id).await
        .expect("Error reading vehicle");
    assert_eq!(vehicle, vehicle_back);

    // UPDATE
    let vehicle = VehicleDescription{
        id: vehicle.id.clone(),
        name: "test_name_2".to_string(),
        color: EntityColor::Cyan,
        vehicle_type: VehicleType::Vtol,
        protocol_id: ProtocolId::MavlinkId{ mav_id: 2 },
        features: vec![VehicleFeatures::Lidar],
        available_modes: vec![VehicleMode::Circle, VehicleMode::RTL]
    };
    let vehicle_back = dao.update("vehicles", vehicle.clone()).await
        .expect("Error updating vehicle");
    assert_eq!(vehicle, vehicle_back);

    // SELECT ALL
    let vehicles: Vec<VehicleDescription> = dao.select_all::<VehicleDescription>("vehicles").await
        .expect("Error reading vehicles");
    assert_eq!(vehicles.len(), 1);
    assert_eq!(vehicles[0], vehicle);

    // SELECT WHERE
    let vehicles_by_protocol_id: Vec<VehicleDescription> = dao.select_where("vehicles", "protocol_id",
        &ProtocolId::MavlinkId{ mav_id: 2 }).await
        .expect("Error reading vehicles by protocol id");
    assert_eq!(vehicles_by_protocol_id.len(), 1);
    assert_eq!(vehicles_by_protocol_id[0], vehicle);

    // DELETE
    dao.delete("vehicles", &vehicle.id).await
        .expect("Error deleting vehicle");

    // TRY TO READ THEN EMPTY
    let vehicle_back = dao.select_one::<VehicleDescription>("vehicles", &vehicle.id).await;
    assert!(vehicle_back.is_err());
}
