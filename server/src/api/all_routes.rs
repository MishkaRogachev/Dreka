use std::net::SocketAddr;
use actix_cors::Cors;
use actix_web::{get, App, HttpServer, web::Data, Responder, HttpResponse};

use crate::models::events::{ClientEvent, ServerEvent};
use crate::{bus::bus, dal::dal};

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json("ok")
}

pub async fn serve(
        dal: dal::Dal,
        server_bus: bus::EventBus::<ServerEvent>,
        client_bus: bus::EventBus::<ClientEvent>,
        address: &SocketAddr
    ) -> anyhow::Result<()> {
    let context = super::context::ApiContext::new(dal, server_bus, client_bus);

    let result = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(ping)
            .service(super::websocket::events_ws)
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
            .service(super::commands::execute_command)
            .service(super::commands::cancel_command)
            .service(super::commands::get_command_execution)
            .service(super::commands::get_command_executions)
            .service(super::missions::create_mission)
            .service(super::missions::upsert_route_item)
            .service(super::missions::remove_route_item)
            .service(super::missions::download_mission)
            .service(super::missions::upload_mission)
            .service(super::missions::clear_mission)
            .service(super::missions::cancel_mission_state)
            .service(super::missions::get_mission)
            .service(super::missions::get_missions)
            .app_data(Data::new(context.clone()))
    }).bind(address)?.run();

    log::info!("Listening REST on {}", address);
    result.await?;
    Ok(())
}
