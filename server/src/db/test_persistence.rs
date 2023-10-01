#[cfg(test)]
use super::persistence;

#[cfg(test)]
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: Option<surrealdb::sql::Thing>,
    pub name: String,
    pub login: String,
    pub records: i32
}

#[tokio::test]
async fn test_crud_operations() {
    let db = persistence::Persistence::new().await.expect("Error establishing a database connection");

    let initial = User {
        id: Option::None,
        name: "John".into(),
        login: "john_123".into(),
        records: 23
    };

    let created = db.create("test_users", &initial).await;
    if let Err(err) = created {
        panic!("Create error: {}", err)
    }
    let mut created = created.unwrap();

    assert_ne!(created.id, None);

    created.records = 17;

    let updated = db.update("test_users", &created).await;
    if let Err(err) = updated {
        panic!("Update error: {}", err)
    }
    let updated = updated.unwrap();
    assert_eq!(created, updated);

    let read = db.read::<User>("test_users", created.id.unwrap().id).await;
    if let Err(err) = read {
        panic!("Read error: {}", err)
    }
    let read = read.unwrap();
    assert!(read.is_some());
    assert_eq!(updated, read.unwrap());

    let removed = db.remove::<User>("test_users", updated.id.unwrap().id).await;
    assert!(removed.is_ok());
}
