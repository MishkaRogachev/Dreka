use std::sync::Arc;
use actix_web::{get, post, delete, web, Responder, HttpResponse};

use crate::{datasource::db, models::communication::{LinkDescription, LinkStatus}};

#[get("/comm/links")]
pub async fn list_descriptions(repo: web::Data<Arc<db::Repository>>) -> impl Responder {
    let result = repo.read_all::<LinkDescription>("link_descriptions").await;

    match result {
        Ok(links) => return HttpResponse::Ok().json(links),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/comm/links/save")]
pub async fn save_description(repo: web::Data<Arc<db::Repository>>, link: web::Json<LinkDescription>) -> impl Responder {
    let link = &link.into_inner();
    let result = repo.create_or_update("link_descriptions", link).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/comm/links/remove/{link_id}")]
pub async fn remove_description(repo: web::Data<Arc<db::Repository>>, path: web::Path<String>) -> impl Responder {
    let id = &path.into_inner();
    let result = repo.remove("link_descriptions", &id).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(id),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/comm/links/status/{link_id}")]
pub async fn get_status(repo: web::Data<Arc<db::Repository>>, path: web::Path<String>) -> impl Responder {
    let result = repo.read::<LinkStatus>("link_statuses", &path.into_inner()).await;

    match result {
        Ok(link_status) => {
            return HttpResponse::Ok().json(link_status);
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
