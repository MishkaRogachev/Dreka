#[cfg(test)]
use super::db;

#[cfg(test)]
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub login: String,
    pub records: i32
}

#[cfg(test)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserStatus {
    pub id: String,
    pub online: bool,
    pub uptime: u64
}

#[tokio::test]
async fn test_repository_crud() {
    let db = db::Repository::new().await.expect("Error establishing a database connection");

    let mut user = User {
        id: "john123".into(),
        name: "John".into(),
        login: "john_123".into(),
        records: 23
    };

    // Create
    let err = db.create("test_users_table", &user).await;
    if let Err(err) = err {
        panic!("Create error: {}", err);
    }

    // Update
    user.name = "Victor".into();
    user.records = 17;
    let err = db.update("test_users_table", &user).await;
    if let Err(err) = err {
        panic!("Update error: {}", err);
    }

    // Read
    let read_user: Result<User, db::DbError> = db.read("test_users_table", &user.id).await;
    match read_user {
        Ok(read_user) => assert_eq!(read_user, user),
        Err(err) => panic!("Read error: {}", err),
    }

    // Delete
    let removed = db.remove("test_users_table", &user.id).await;
    assert!(removed.is_ok());
}

#[tokio::test]
async fn test_repository_upsert() {
    let db = db::Repository::new().await.expect("Error establishing a database connection");

    let user1 = User {
        id: "0x122333123455".into(),
        name: "John".into(),
        login: "john_123".into(),
        records: 23
    };

    let mut user2 = User {
        id: "0x123045684377".into(),
        name: "Vital".into(),
        login: "tital_work".into(),
        records: 17
    };


    db.upsert("test_users_table", &user1).await.expect("Failed to create user");
    db.upsert("test_users_table", &user2).await.expect("Failed to create user");

    user2.records = 19;
    db.upsert("test_users_table", &user2).await.expect("Failed to update user");

    // Read All
    let read_all = db.read_all::<User>("test_users_table").await;
    match read_all {
        Ok(read_all) => {
            assert_eq!(read_all.len(), 2);
            assert_eq!(read_all[0], user1);
            assert_eq!(read_all[1], user2);
        },
        Err(err) => panic!("Read all error: {}", err),
    }
}