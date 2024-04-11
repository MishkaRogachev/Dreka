mod db;
mod models;
mod bus;
mod dal;
mod services;
mod api;

use std::net;

use crate::models::events::{ServerEvent, ClientEvent};

const DEFAULT_REST_ADDRESS: net::SocketAddr = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 45486);
const DATABASE_NAME: &str = "dreka";
const DATABASE_NAMESPACE_NAME: &str = "dreka";

pub async fn start() -> anyhow::Result<()> {
    let colors = fern::colors::ColoredLevelConfig::new()
        .error(fern::colors::Color::Red)
        .warn(fern::colors::Color::Yellow)
        .info(fern::colors::Color::Green);

    fern::Dispatch::new()
        .filter(|metadata| {
            metadata.target().starts_with("brygge")
        })
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{timestamp}] - {level} - {message}",
                timestamp = chrono::Local::now().format("%d.%m.%Y-%H:%M:%S"),
                level = colors.color(record.level()),
                message = message
            ))
        })
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    log::info!("Starting Brygge server..");

    let db = surrealdb::Surreal::new::<surrealdb::engine::local::Mem>(()).await?;
    db.use_ns(DATABASE_NAME).use_db(DATABASE_NAMESPACE_NAME).await?;
    let dao = db::surreal_dao::Dao::new(db);

    let server_bus = bus::bus::EventBus::<ServerEvent>::new();
    let client_bus = bus::bus::EventBus::<ClientEvent>::new();
    let repository = dal::dal::Dal::new(dao, server_bus.clone());

    let mut comm_service = services::communication::service::Service::new(
        repository.clone(),
        server_bus.clone(),
        client_bus.clone()
    );

    tokio::select! {
        result = comm_service.start() => {
            match result {
                Ok(()) => {},
                Err(err) => log::error!("Communication service start error: {}", err),
            }
        }
        _ = api::all_routes::serve(repository, server_bus, client_bus, &DEFAULT_REST_ADDRESS) => {}
        _ = tokio::signal::ctrl_c() => {}
    }
    Ok(())
}
