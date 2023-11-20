use std::net::SocketAddr;
use actix_cors::Cors;
use actix_web::{get, App, HttpServer, web::Data, Responder, HttpResponse};

use super::shared::Shared;

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json("ok")
}

pub async fn serve(shared: Shared, address: &SocketAddr) -> std::io::Result<()> {
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
            .service(super::telemetry::get_flight_data)
            .service(super::telemetry::get_sns_data)
            .service(super::telemetry::get_sensors_data)
            .app_data(Data::new(shared.clone()))
    }).bind(address)?.run();

    println!("Listening REST on {}", address);
    return result.await;
}
