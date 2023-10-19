use actix_web::{get, post, web, Responder, HttpResponse};

use crate::models::vehicles::VehicleDescription;
use super::shared::Shared;

#[get("/vehicles")]
pub async fn list_vehicles(shared: web::Data<Shared>) -> impl Responder {
    let response = shared.repository.read_all::<VehicleDescription>("vehicles").await;
    match response {
        Ok(vehicles) => return HttpResponse::Ok().json(vehicles),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/vehicles/create")]
pub async fn add_vehicle(shared: web::Data<Shared>, new_vehicle: web::Json<VehicleDescription>) -> impl Responder {
    let result = shared.repository.create("vehicles", &new_vehicle.into_inner()).await;
    match result {
        Ok(vehicle) => HttpResponse::Ok().json(vehicle),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}