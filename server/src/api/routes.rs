use std::net::SocketAddr;
use actix_cors::Cors;
use actix_web::{get, App, HttpServer, web::Data, Responder, HttpResponse};

use crate::{models::{events::ClentEvent, telemetry::VehicleTelemetry}, registry::registry};

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json("ok")
}

pub async fn serve(
        registry: registry::Registry,
        client_events_tx: flume::Sender<ClentEvent>,
        telemetry_rx: flume::Receiver<VehicleTelemetry>,
        address: &SocketAddr
    ) -> anyhow::Result<()> {
    let context = super::context::ApiContext::new(registry, client_events_tx, telemetry_rx);

    let result = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(ping)
            .service(super::communication::get_descriptions)
            .service(super::communication::get_description)
            .service(super::communication::get_status)
            .service(super::communication::get_statuses)
            .service(super::communication::post_link)
            .service(super::communication::delete_link)
            .service(super::communication::set_link_connected)
            .service(super::vehicles::get_descriptions)
            .service(super::vehicles::get_description)
            .service(super::vehicles::get_status)
            .service(super::vehicles::get_statuses)
            .service(super::vehicles::post_vehicle)
            .service(super::vehicles::delete_vehicle)
            .service(super::telemetry::telemetry_ws)
            .app_data(Data::new(context.clone()))
    }).bind(address)?.run();

    log::info!("Listening REST on {}", address);
    result.await?;
    Ok(())
}
