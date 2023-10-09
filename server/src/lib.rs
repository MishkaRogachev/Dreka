mod datasource;
mod models;
mod protocols;
mod routes;

use std::sync::Arc;
use std::net;

use tokio::signal;

const DEFAULT_REST_ADDRESS: net::SocketAddr = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 45486);

pub async fn start() -> std::io::Result<()> {
    println!("Starting Brygge server..");

    // TODO: separate temp and persistent databases
    let db = Arc::new(datasource::db::Repository::new().await
        .expect("Error establishing a database connection"));

    protocols::links::check_and_create_links(&db).await?;
    let hub = tokio::spawn(protocols::hub::start(db.clone()));

    let rest = routes::root::serve(db, &DEFAULT_REST_ADDRESS);

    tokio::select! {
        _ = hub => {}
        _ = rest => {}
        _ = signal::ctrl_c() => {}
    }
    Ok(())
}
