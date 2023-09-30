mod db;
mod models;
mod protocols;
mod routes;

use std::sync::Arc;
use std::net;

pub async fn start() -> std::io::Result<()> {
    println!("Starting Brygge server..");

    let persistence = Arc::new(db::persistence::Persistence::new().await
        .expect("Error establishing a database connection"));

    let hub = protocols::hub::Hub::new(persistence.clone());
    hub.start().await?;


    return routes::root::serve(persistence,
        &net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 45486)).await;
}
