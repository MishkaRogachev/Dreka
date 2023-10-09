use crate::{datasource::db, models::vehicles::VehicleDescription};

use std::sync::Arc;
use actix_web::{get, post, web, Responder, HttpResponse};

#[get("/vehicles")]
pub async fn list_vehicles(repo: web::Data<Arc<db::Repository>>) -> impl Responder {
    let response = repo.read_all::<VehicleDescription>("vehicles").await;
    match response {
        Ok(vehicles) => return HttpResponse::Ok().json(vehicles),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/vehicles/create")]
pub async fn add_vehicle(repo: web::Data<Arc<db::Repository>>, new_vehicle: web::Json<VehicleDescription>) -> impl Responder {
    let result = repo.create("vehicles", &new_vehicle.into_inner()).await;
    match result {
        Ok(vehicle) => HttpResponse::Ok().json(vehicle),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}