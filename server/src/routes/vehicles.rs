use crate::{db::persistence, models::vehicles::VehicleDescription};

use std::sync::Arc;
use actix_web::{get, post, web, Responder, HttpResponse};

#[get("/vehicles")]
pub async fn list_vehicles(persistence: web::Data<Arc<persistence::Persistence>>) -> impl Responder {
    let response = persistence.read_all::<VehicleDescription>("vehicles").await;
    match response {
        Ok(vehicles) => return HttpResponse::Ok().json(vehicles),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/vehicles/create")]
pub async fn add_vehicle(persistence: web::Data<Arc<persistence::Persistence>>, new_vehicle: web::Json<VehicleDescription>) -> impl Responder {
    let result = persistence.create("vehicles", &new_vehicle.into_inner()).await;
    match result {
        Ok(vehicle) => HttpResponse::Ok().json(vehicle),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}