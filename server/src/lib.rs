mod persistence;
mod models;
mod services;
mod api;
mod context;

use std::net;
use tokio::sync::broadcast::channel;

const DEFAULT_REST_ADDRESS: net::SocketAddr = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 45486);
const CHANNEL_CAPACITY: usize = 100;

pub async fn start() -> anyhow::Result<()> {
    println!("Starting Brygge server..");

    let db = surrealdb::Surreal::new::<surrealdb::engine::local::Mem>(()).await?;
    db.use_ns("dreka").use_db("dreka").await?;

    let (tx, rx) = channel::<models::events::ClentEvent>(CHANNEL_CAPACITY);
    let context = context::AppContext::new(db, tx);

    let mut comm_service = services::communication::service::Service::new(context.clone(), rx);

    tokio::select! {
        result = comm_service.start() => {
            match result {
                Ok(()) => {},
                Err(err) => println!("Communication service start error: {}", err),
            }
        }
        _ = api::routes::serve(context, &DEFAULT_REST_ADDRESS) => {}
        _ = tokio::signal::ctrl_c() => {}
    }
    Ok(())
}
