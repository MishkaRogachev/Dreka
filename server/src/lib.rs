mod models;
mod persistence;
mod registry;
mod services;
mod api;

use std::net;
use flume;

const DEFAULT_REST_ADDRESS: net::SocketAddr = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 45486);
const DATABASE_NAME: &str = "dreka";
const DATABASE_NAMESPACE_NAME: &str = "dreka";
const CHANNEL_CAPACITY: usize = 100;

pub async fn start() -> anyhow::Result<()> {
    let colors = fern::colors::ColoredLevelConfig::new()
        .error(fern::colors::Color::Red)
        .warn(fern::colors::Color::Yellow)
        .info(fern::colors::Color::Green);

    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
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

    let registry = registry::registry::Registry::new(db);
    let (client_events_tx, client_events_rx) = flume::bounded(CHANNEL_CAPACITY);
    let (telemetry_tx, telemetry_rx) = flume::bounded(CHANNEL_CAPACITY);

    let mut comm_service = services::communication::service::Service::new(
        registry.clone(),
        client_events_rx,
        telemetry_tx,
        telemetry_rx.clone()
    );

    tokio::select! {
        result = comm_service.start() => {
            match result {
                Ok(()) => {},
                Err(err) => log::error!("Communication service start error: {}", err),
            }
        }
        _ = api::routes::serve(registry, client_events_tx, telemetry_rx, &DEFAULT_REST_ADDRESS) => {}
        _ = tokio::signal::ctrl_c() => {}
    }
    Ok(())
}
