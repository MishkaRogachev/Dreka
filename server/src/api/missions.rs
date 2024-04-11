use actix_web::{get, post, put, delete, web, Responder, HttpResponse};

use crate::models::{events::ClientEvent, missions::*, vehicles::VehicleId};
use super::context::ApiContext;

#[post("/missions/create")]
pub async fn create_mission(context: web::Data<ApiContext>, vehicle_id: web::Json<VehicleId>) -> impl Responder {
    let vehicle_id = vehicle_id.into_inner();
    let result = context.dal.create_new_mission(&vehicle_id).await;

    match result {
        Ok(mission) => HttpResponse::Ok().json(mission),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[post("/missions/{mission_id}/upsert_route_item/{index}")]
pub async fn upsert_route_item(context: web::Data<ApiContext>, path: web::Path<(MissionId, u16)>, item: web::Json<MissionRouteItem>) -> impl Responder {
    let (mission_id, index) = path.into_inner();
    let item = item.into_inner();
    let result = context.dal.upsert_route_item(&mission_id, item, index).await;

    match result {
        Ok(mission) => HttpResponse::Ok().json(mission),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[delete("/missions/{mission_id}/remove_route_item/{index}")]
pub async fn remove_route_item(context: web::Data<ApiContext>, path: web::Path<(MissionId, u16)>) -> impl Responder {
    let (mission_id, index) = path.into_inner();
    let result = context.dal.remove_route_item(&mission_id, index).await;

    match result {
        Ok(mission) => HttpResponse::Ok().json(mission),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[put("/missions/download/{mission_id}")]
pub async fn download_mission(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let mission_id: MissionId = path.into_inner();

    match context.client_bus.publish(ClientEvent::DownloadMission { mission_id: mission_id.clone() } ) {
        Ok(_) => HttpResponse::Ok().json(mission_id),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[put("/missions/upload/{mission_id}")]
pub async fn upload_mission(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let mission_id: MissionId = path.into_inner();

    match context.client_bus.publish(ClientEvent::UploadMission { mission_id: mission_id.clone() } ) {
        Ok(_) => HttpResponse::Ok().json(mission_id),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[delete("/missions/clear/{mission_id}")]
pub async fn clear_mission(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let mission_id: MissionId = path.into_inner();

    let mission = context.dal.mission(&mission_id).await;
    if let Err(err) = mission {
        log::warn!("REST: error {}", &err);
        return HttpResponse::InternalServerError().json(err.to_string());
    }
    let mut mission = mission.unwrap();

    mission.route.items.clear();

    let mission = context.dal.update_mission(mission).await;
    if let Err(err) = mission {
        log::warn!("REST: error {}", &err);
        return HttpResponse::InternalServerError().json(err.to_string());
    }
    let mission = mission.unwrap();

    match context.client_bus.publish(ClientEvent::ClearMission { mission_id } ) {
        Ok(_) => HttpResponse::Ok().json(mission),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[put("/missions/cancel/{mission_id}")]
pub async fn cancel_mission_state(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let mission_id: MissionId = path.into_inner();

    match context.client_bus.publish(ClientEvent::CancelMissionState { mission_id: mission_id.clone() } ) {
        Ok(_) => HttpResponse::Ok().json(mission_id),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/missions/mission/{mission_id}")]
pub async fn get_mission(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let mission_id: MissionId = path.into_inner();
    let result = context.dal.mission(&mission_id).await;

    match result {
        Ok(mission) => return HttpResponse::Ok().json(mission),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/missions/missions")]
pub async fn get_missions(context: web::Data<ApiContext>) -> impl Responder {
    let result = context.dal.all_missions().await;

    match result {
        Ok(missions) => return HttpResponse::Ok().json(missions),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}
