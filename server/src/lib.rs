mod datasource;
mod models;
mod services;
mod api;

use std::sync::Arc;
use std::net;

const DEFAULT_REST_ADDRESS: net::SocketAddr = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 45486);

pub async fn start() -> std::io::Result<()> {
    println!("Starting Brygge server..");

    let repository = datasource::db::Repository::new().await
        .expect("Error establishing a database connection");
    let repository = Arc::new(repository);

    let (tx, rx) = tokio::sync::broadcast::channel::<models::events::ClentEvent>(100);

    let mut comm_service = services::communication::service::Service::new(repository.clone(), rx);

    let shared = api::shared::Shared::new(repository, tx);

    tokio::select! {
        _ = comm_service.start() => {}
        _ = api::root::serve(shared, &DEFAULT_REST_ADDRESS) => {}
        _ = tokio::signal::ctrl_c() => {}
    }
    Ok(())
}
