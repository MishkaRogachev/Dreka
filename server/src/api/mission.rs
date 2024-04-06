use actix_web::{get, put, delete, web, Responder, HttpResponse};

use crate::models::{events::ClientEvent, mission::*};
use super::context::ApiContext;

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

    match context.client_bus.publish(ClientEvent::ClearMission { mission_id: mission_id.clone() } ) {
        Ok(_) => HttpResponse::Ok().json(mission_id),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[put("/missions/cancel/{mission_id}")]
pub async fn cancel_mission_state(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let mission_id: MissionId = path.into_inner();

    match context.client_bus.publish(ClientEvent::DownloadMission { mission_id: mission_id.clone() } ) {
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
    let result = context.registry.missions.mission(&mission_id).await;

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
    let result = context.registry.missions.all_missions().await;

    match result {
        Ok(missions) => return HttpResponse::Ok().json(missions),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}
