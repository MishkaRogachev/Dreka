use actix_web::{get, post, delete, web, Responder, HttpResponse};

use crate::models::vehicles::{VehicleId, VehicleDescription};
use crate::context::AppContext;

#[get("/vehicles/description/{vehicle_id}")]
pub async fn get_description(context: web::Data<AppContext>, path: web::Path<String>) -> impl Responder {
    let vehicle_id: VehicleId = path.into_inner();
    let result = context.vehicles.vehicle(&vehicle_id).await;

    match result {
        Ok(vehicle) => return HttpResponse::Ok().json(vehicle),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/vehicles/descriptions")]
pub async fn get_descriptions(context: web::Data<AppContext>) -> impl Responder {
    let result = context.vehicles.all_vehicles().await;

    match result {
        Ok(vehicles) => return HttpResponse::Ok().json(vehicles),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/vehicles/status/{vehicle_id}")]
pub async fn get_status(context: web::Data<AppContext>, path: web::Path<String>) -> impl Responder {
    let vehicle_id: VehicleId = path.into_inner();
    let result = context.vehicles.status(&vehicle_id).await;

    match result {
        Ok(status) => return HttpResponse::Ok().json(status),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            return HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/vehicles/statuses")]
pub async fn get_statuses(context: web::Data<AppContext>) -> impl Responder {
    let result = context.vehicles.all_statuses().await;

    match result {
        Ok(statuses) => return HttpResponse::Ok().json(statuses),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[post("/vehicles/save")]
pub async fn post_vehicle(context: web::Data<AppContext>, vehicle: web::Json<VehicleDescription>) -> impl Responder {
    let vehicle = vehicle.into_inner();
    let result = context.vehicles.save_vehicle( &vehicle).await;

    match result {
        Ok(vehicle) => HttpResponse::Ok().json(vehicle),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[delete("/vehicles/remove/{vehicle_id}")]
pub async fn delete_vehicle(context: web::Data<AppContext>, path: web::Path<String>) -> impl Responder {
    let vehicle_id: VehicleId = path.into_inner();

    let result = context.vehicles.delete_vehicle(&vehicle_id).await;
    if let Err(err) = result {
        log::warn!("REST error: {}", &err); // TODO: add path here
        return HttpResponse::InternalServerError().json(err.to_string())
    }
    HttpResponse::Ok().json(vehicle_id)
}
