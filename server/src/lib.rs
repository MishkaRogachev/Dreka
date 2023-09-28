mod db;
mod models;
mod routes;

pub async fn start() -> std::io::Result<()> {
    println!("Starting Brygge server");

    let persistence = db::persistence::Persistence::new().await
        .expect("Error establishing a database connection");

    return routes::root::serve(persistence).await;
}
