use actix_web::{get, post, delete, web, Responder, HttpResponse};

use crate::models::vehicles::{VehicleId, VehicleDescription};
use super::context::ApiContext;

#[post("/vehicles/save")]
pub async fn post_vehicle(context: web::Data<ApiContext>, vehicle: web::Json<VehicleDescription>) -> impl Responder {
    let vehicle = vehicle.into_inner();
    let result = context.dal.save_vehicle(vehicle).await;

    match result {
        Ok(vehicle) => HttpResponse::Ok().json(vehicle),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[delete("/vehicles/remove/{vehicle_id}")]
pub async fn delete_vehicle(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let vehicle_id: VehicleId = path.into_inner();

    if let Err(err) = context.dal.delete_vehicle(&vehicle_id).await {
        log::warn!("REST error: {}", &err); // TODO: add path here
        return HttpResponse::InternalServerError().json(err.to_string())
    }
    HttpResponse::Ok().json(vehicle_id)
}

#[get("/vehicles/description/{vehicle_id}")]
pub async fn get_description(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let vehicle_id: VehicleId = path.into_inner();
    let result = context.dal.vehicle(&vehicle_id).await;

    match result {
        Ok(vehicle) => return HttpResponse::Ok().json(vehicle),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/vehicles/descriptions")]
pub async fn get_descriptions(context: web::Data<ApiContext>) -> impl Responder {
    let result = context.dal.all_vehicles().await;

    match result {
        Ok(vehicles) => return HttpResponse::Ok().json(vehicles),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/vehicles/status/{vehicle_id}")]
pub async fn get_status(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let vehicle_id: VehicleId = path.into_inner();
    let result = context.dal.vehcile_status(&vehicle_id).await;

    match result {
        Ok(status) => return HttpResponse::Ok().json(status),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            return HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/vehicles/statuses")]
pub async fn get_statuses(context: web::Data<ApiContext>) -> impl Responder {
    let result = context.dal.all_vehicles_statuses().await;

    match result {
        Ok(statuses) => return HttpResponse::Ok().json(statuses),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}
