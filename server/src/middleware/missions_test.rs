#[cfg(test)]
mod tests {
    use surrealdb::{engine::local::Mem, Surreal};

    use crate::middleware::{bus::EventBus, missions::Persistence};
    use crate::models::missions::{Mission, MissionId, MissionRouteItem};
    use crate::models::spatial::{Geodetic, GeodeticFrame};
    use crate::models::{events::ServerEvent, vehicles::VehicleId};

    async fn setup() -> (Persistence, tokio::sync::broadcast::Receiver<ServerEvent>) {
        let db = Surreal::new::<Mem>(())
            .await
            .expect("Error establishing a database connection");
        db.use_ns("test").use_db("test").await.expect("Error setting namespace and database");

        let bus = EventBus::<ServerEvent>::new();
        (Persistence::new(db, bus.clone()), bus.subscribe())
    }

    async fn create_new_mission(
        persistence: &Persistence,
        rx: &mut tokio::sync::broadcast::Receiver<ServerEvent>,
        vehicle_id: &VehicleId
    ) -> Mission {
        let created_mission = persistence.create_new_mission(&vehicle_id).await
            .expect("Error creating mission");
        assert_ne!(created_mission.id.len(), 0);

        match rx.recv().await.expect("Error receiving event") {
            ServerEvent::MissionUpserted{ mission } => {
                assert_eq!(created_mission, mission);
            },
            _ => panic!("Unexpected event")
        }
        created_mission
    }

    async fn upsert_route_item(
        persistence: &Persistence,
        rx: &mut tokio::sync::broadcast::Receiver<ServerEvent>,
        mission_id: &MissionId,
        item: MissionRouteItem,
        index: u16
    ) -> MissionRouteItem {
        let route = persistence.mission_route(&mission_id).await
            .expect("Error reading mission route");

        let update_only = index < route.items.len() as u16;
        let items_to_be_inserted = if update_only {
            1
        } else {
            index as usize - route.items.len() + 1
        };

        let response = persistence.upsert_route_item(&mission_id, item.clone(), index)
            .await.expect("Error setting route item");
        assert_eq!(response.len(), items_to_be_inserted);

        for i in 0..items_to_be_inserted {
            let expected_index = if update_only {
                index
            } else {
                (route.items.len() + i) as u16
            };
            let expected_item = if expected_index == index {
                item.clone()
            } else {
                MissionRouteItem::Gap {}
            };
            assert_eq!(response[i].0, expected_index);
            assert_eq!(response[i].1, expected_item);

            match rx.recv().await.expect("Error receiving event") {
                ServerEvent::MissionRouteItemUpserted{ mission_id, index: idx, item: item_back } => {
                    assert_eq!(mission_id, mission_id);
                    assert_eq!(idx, expected_index);
                    assert_eq!(expected_item, item_back);
                },
                _ => panic!("Unexpected event")
            }
        }

        item
    }

    #[tokio::test]
    async fn test_crud_mission() {
        let (persistence, mut rx) = setup().await;
        let vehicle_id = "mav_1".to_string();

        let mission = create_new_mission(&persistence, &mut rx, &vehicle_id).await;

        let mission_back = persistence.mission(&mission.id).await
            .expect("Error reading mission");
        assert_eq!(mission, mission_back);

        persistence.mission_route(&mission.id).await
            .expect("Route must exist for created mission");
        persistence.mission_status(&mission.id).await
            .expect("Status must exist for created mission");

        let vehicle_mission = persistence.mission_for_vehicle(&vehicle_id).await
            .expect("Error reading mission for vehicle");
        assert_eq!(mission.id, vehicle_mission.unwrap().id);

        persistence.delete_mission(&mission.id).await.expect("Error deleting mission");

        match rx.recv().await.expect("Error receiving event") {
            ServerEvent::MissionRemoved{ mission_id } => {
                assert_eq!(mission.id, mission_id);
            },
            _ => panic!("Unexpected event")
        }

        let mission_back = persistence.mission(&mission.id).await;
        assert!(mission_back.is_err());
    }

    #[tokio::test]
    async fn test_upsert_route_item() {
        let (persistence, mut rx) = setup().await;
        let vehicle_id = "mav_1".to_string();

        let mission_id = create_new_mission(&persistence, &mut rx, &vehicle_id).await.id;

        let first = upsert_route_item(&persistence, &mut rx, &mission_id,
            MissionRouteItem::Takeoff {
                position: Geodetic {
                    latitude: 45.524545,
                    longitude: 56.6345,
                    altitude: 300.00,
                    frame: GeodeticFrame::Wgs84AboveSeaLevel
                },
                pitch: 15.0,
                yaw: None
            },
            0
        ).await;

        let second = upsert_route_item(&persistence, &mut rx, &mission_id,
            MissionRouteItem::Waypoint {
                position: Geodetic {
                    latitude: 45.2455,
                    longitude: 56.32452,
                    altitude: 452.45,
                    frame: GeodeticFrame::Wgs84AboveSeaLevel
                },
                hold: 0,
                pass_radius: 35.23,
                accept_radius: 12.45,
                yaw: None
            },
            1
        ).await;

        let last = upsert_route_item(&persistence, &mut rx, &mission_id,
            MissionRouteItem::Landing {
                position: Geodetic {
                    latitude: 42.3715,
                    longitude: 51.4212,
                    altitude: 0.00,
                    frame: GeodeticFrame::Wgs84RelativeHome
                },
                abort_altitude: Some(150.0),
                yaw: Some(36)
            },
            4
        ).await;

        let fill_gap = upsert_route_item(&persistence, &mut rx, &mission_id,
            MissionRouteItem::LoiterTrn {
                position: Geodetic {
                    latitude: 42.6734,
                    longitude: 51.7694,
                    altitude: 200.00,
                    frame: GeodeticFrame::Wgs84RelativeHome
                },
                heading_required: true,
                radius: 550.0,
                turns: 3,
                clockwise: false
            },
            3
        ).await;

        let gap = MissionRouteItem::Gap {};
        let route = persistence.mission_route(&mission_id).await
            .expect("Error reading mission route");
        assert_eq!(route.items.len(), 5);
        assert_eq!(route.items[0], first);
        assert_eq!(route.items[1], second);
        assert_eq!(route.items[2], gap);
        assert_eq!(route.items[3], fill_gap);
        assert_eq!(route.items[4], last);
    }
}
