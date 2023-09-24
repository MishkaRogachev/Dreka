use crate::{db::persistence, models::vehicles::Vehicle};

use actix_web::{get, web, Responder, HttpResponse};

#[get("/vehicles")]
pub async fn list_vehicles(data: web::Data<persistence::Persistence>) -> impl Responder {
    let response = data.read_all::<Vehicle>("vehicles").await;
    match response {
        Ok(vehicles) => return HttpResponse::Ok().json(vehicles),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}