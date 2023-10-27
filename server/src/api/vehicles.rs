use actix_web::{get, post, delete, web, Responder, HttpResponse};

use crate::models::vehicles::{VehicleDescription, VehicleStatus};
use super::shared::Shared;

#[get("/vehicles")]
pub async fn list_descriptions(shared: web::Data<Shared>) -> impl Responder {
    let result = shared.repository.read_all::<VehicleDescription>("vehicle_descriptions").await;

    match result {
        Ok(vehicles) => return HttpResponse::Ok().json(vehicles),
        Err(err) => {
            println!("REST error: {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/vehicles/{vehicle_id}")]
pub async fn vehicle_description(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let id = &path.into_inner();
    let result = shared.repository.read::<VehicleDescription>("vehicle_descriptions", id).await;

    match result {
        Ok(vehicles) => return HttpResponse::Ok().json(vehicles),
        Err(err) => {
            println!("REST error: {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[post("/vehicles/save")]
pub async fn save_description(shared: web::Data<Shared>, vehicle: web::Json<VehicleDescription>) -> impl Responder {
    let vehicle = vehicle.into_inner();
    let result = shared.repository.upsert("vehicle_descriptions", &vehicle).await;

    match result {
        Ok(vehicle) => {
            HttpResponse::Ok().json(vehicle)
        },
        Err(err) => {
            println!("REST error: {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[delete("/vehicles/remove/{vehicle_id}")]
pub async fn remove_description(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let vehicle_id = &path.into_inner();
    let result = shared.repository.remove("vehicle_descriptions", &vehicle_id).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(vehicle_id),
        Err(err) => {
            println!("REST error: {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/vehicles/status/{vehicle_id}")]
pub async fn get_status(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let vehicle_id: String = path.into_inner();
    let result = shared.repository.read::<VehicleStatus>("vehicle_statuses", &vehicle_id).await;

    match result {
        Ok(status) => {
            return HttpResponse::Ok().json(status);
        },
        Err(err) => {
            if let crate::datasource::db::DbError::NoData = err {
                return HttpResponse::Ok().json(VehicleStatus::default_for_id(&vehicle_id))
            }
            return HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/vehicles/statuses/{vehicle_ids}")]
pub async fn get_statuses(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let mut result = Vec::<VehicleStatus>::new();

    for vehicle_id in path.into_inner().split(",") {
        let status = shared.repository.read::<VehicleStatus>("vehicle_statuses", vehicle_id).await;
        match status {
            Ok(status) => {
                result.push(status);
            },
            Err(err) => {
                if let crate::datasource::db::DbError::NoData = err {
                    // skip
                } else {
                    return HttpResponse::InternalServerError().json(err.to_string());
                }
            }
        }
    }
    return HttpResponse::Ok().json(result);
}

