#[cfg(test)]
use super::persistence;

#[cfg(test)]
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub login: String,
    pub records: i32
}

#[cfg(test)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Permission {
    pub code: u16,
    pub tip: String
}

#[tokio::test]
async fn test_crud_operations() {
    let db = persistence::Persistence::new().await.expect("Error establishing a database connection");

    let john = User {
        name: "John".into(),
        login: "john_123".into(),
        records: 23
    };

    let created = db.create("test_users", "0x123", &john).await;
    assert!(created.is_ok());
    assert_eq!(john, created.unwrap());

    let edit_route = Permission {
        code: 0x1223,
        tip: "edit_route".into()
    };

    let updated = db.update("test_users", "0x123", &edit_route).await;
    assert!(updated.is_ok());
    assert_eq!(edit_route, updated.unwrap());

    let john_db = db.read::<User>("test_users", "0x123").await;
    assert!(john_db.is_ok());
    let john_db = john_db.unwrap();
    assert!(john_db.is_some());
    assert_eq!(john, john_db.unwrap());

    let edit_route_db = db.read::<Permission>("test_users", "0x123").await;
    assert!(edit_route_db.is_ok());
    let edit_route_db = edit_route_db.unwrap();
    assert!(edit_route_db.is_some());
    assert_eq!(edit_route, edit_route_db.unwrap());

    let removed = db.remove::<User>("test_users", "0x123").await;
    assert!(removed.is_ok());
}
