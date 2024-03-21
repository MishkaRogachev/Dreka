mod persistence;
mod models;
mod services;
mod api;
mod context;

use std::net;
use fern::colors::{Color, ColoredLevelConfig};
use tokio::sync::broadcast::channel;

const DEFAULT_REST_ADDRESS: net::SocketAddr = net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 45486);
const CHANNEL_CAPACITY: usize = 100;
const DATABASE_NAME: &str = "dreka";
const DATABASE_NAMESPACE_NAME: &str = "dreka";

pub async fn start() -> anyhow::Result<()> {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green);

    fern::Dispatch::new()
        .level(log::LevelFilter::Info)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{timestamp}]{level}({target}):{message}",
                timestamp = chrono::Local::now().format("%d.%m.%Y-%H:%M:%S"),
                level = colors.color(record.level()),
                target = record.target(),
                message = message
            ))
        })
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    log::info!("Starting Brygge server..");

    let db = surrealdb::Surreal::new::<surrealdb::engine::local::Mem>(()).await?;
    db.use_ns(DATABASE_NAME).use_db(DATABASE_NAMESPACE_NAME).await?;

    let (tx, rx) = channel::<models::events::ClentEvent>(CHANNEL_CAPACITY);
    let context = context::AppContext::new(db, tx);

    let mut comm_service = services::communication::service::Service::new(context.clone(), rx);

    tokio::select! {
        result = comm_service.start() => {
            match result {
                Ok(()) => {},
                Err(err) => log::error!("Communication service start error: {}", err),
            }
        }
        _ = api::routes::serve(context, &DEFAULT_REST_ADDRESS) => {}
        _ = tokio::signal::ctrl_c() => {}
    }
    Ok(())
}
