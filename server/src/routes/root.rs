use crate::db::persistence;

use actix_web::{get, App, HttpServer, web::Data, Responder, HttpResponse};

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Dreka server is online")
}

pub async fn serve(persistence: persistence::Persistence) -> std::io::Result<()> {
    HttpServer::new(move || {
    App::new()
        .service(ping)
        .service(super::vehicles::list_vehicles)
        .app_data(Data::new(persistence.clone()))
    }).bind(("127.0.0.1", 45486))?.run().await
}