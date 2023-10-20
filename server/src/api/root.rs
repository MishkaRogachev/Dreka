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
            .service(super::vehicles::list_descriptions)
            .service(super::vehicles::vehicle_description)
            .service(super::vehicles::save_description)
            .service(super::vehicles::remove_description)
            .service(super::vehicles::get_status)
            .service(super::communication::list_descriptions)
            .service(super::communication::link_description)
            .service(super::communication::save_description)
            .service(super::communication::remove_description)
            .service(super::communication::get_status)
            .service(super::communication::set_link_enabled)
            .app_data(Data::new(shared.clone()))
    }).bind(address)?.run();

    println!("Listening REST on {}", address);
    return result.await;
}
