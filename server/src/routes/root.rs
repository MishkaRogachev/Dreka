use crate::datasource::db;

use std::{sync::Arc, net::SocketAddr};
use actix_cors::Cors;
use actix_web::{get, App, HttpServer, web::Data, Responder, HttpResponse};

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json("ok")
}

pub async fn serve(repo: Arc<db::Repository>, address: &SocketAddr) -> std::io::Result<()> {
    let result = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(ping)
            .service(super::vehicles::list_vehicles)
            .service(super::vehicles::add_vehicle)
            .service(super::communication::list_descriptions)
            .service(super::communication::save_description)
            .service(super::communication::remove_description)
            .service(super::communication::get_status)
            .app_data(Data::new(repo.clone()))
    }).bind(address)?.run();

    println!("Listening REST on {}", address);
    return result.await;
}
