use crate::db::persistence;

use actix_cors::Cors;
use actix_web::{get, App, HttpServer, web::Data, Responder, HttpResponse};

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().json("ok")
}

pub async fn serve(persistence: persistence::Persistence) -> std::io::Result<()> {
    let result = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(ping)
            .service(super::vehicles::list_vehicles)
            .service(super::vehicles::add_vehicle)
            .app_data(Data::new(persistence.clone()))
    }).bind(("127.0.0.1", 45486))?.run();

    return result.await;
}
