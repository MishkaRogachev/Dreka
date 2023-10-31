use actix_web::{get, web, Responder, HttpResponse};

use crate::models::telemetry::{FlightData, SnsData};
use super::shared::Shared;

#[get("/telemetry/flight/{vehicle_id}")]
pub async fn get_flight_data(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let id = &path.into_inner();
    let result = shared.repository.read::<FlightData>("vehicle_flight_data", id).await;

    match result {
        Ok(vehicle) => return HttpResponse::Ok().json(vehicle),
        Err(err) => {
            println!("REST(/telemetry/flight/{}): error {}", &id, &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/telemetry/sns/{vehicle_id}")]
pub async fn get_sns_data(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let id = &path.into_inner();
    let result = shared.repository.read::<SnsData>("vehicle_sns_data", id).await;

    match result {
        Ok(vehicle) => return HttpResponse::Ok().json(vehicle),
        Err(err) => {
            println!("REST(/telemetry/sns/{}): error {}", &id, &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}