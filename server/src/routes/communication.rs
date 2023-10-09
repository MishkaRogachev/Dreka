use std::sync::Arc;
use actix_web::{get, post, delete, web, Responder, HttpResponse};

use crate::{datasource::db, models::communication::{LinkDescription, LinkStatus}};

#[get("/comm/links")]
pub async fn list_descriptions(repo: web::Data<Arc<db::Repository>>) -> impl Responder {
    let response = repo.read_all::<LinkDescription>("link_descriptions").await;
    match response {
        Ok(links) => return HttpResponse::Ok().json(links),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/comm/links/create")]
pub async fn add_description(repo: web::Data<Arc<db::Repository>>, new_link: web::Json<LinkDescription>) -> impl Responder {
    let result = repo.create("link_descriptions", &new_link.into_inner()).await;
    match result {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/comm/links/update")]
pub async fn update_description(repo: web::Data<Arc<db::Repository>>, link: web::Json<LinkDescription>) -> impl Responder {
    let result = repo.update("link_descriptions", &link.into_inner()).await;
    match result {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/comm/links/remove/{link_id}")]
pub async fn remove_description(repo: web::Data<Arc<db::Repository>>, path: web::Path<String>) -> impl Responder {
    let result = repo.remove("link_descriptions", &path.into_inner()).await;
    match result {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/comm/links/status/{link_id}")]
pub async fn get_status(repo: web::Data<Arc<db::Repository>>, path: web::Path<String>) -> impl Responder {
    let response = repo.read::<LinkStatus>("link_statuses", &path.into_inner()).await;

    match response {
        Ok(link_status) => {
            return HttpResponse::Ok().json(link_status);
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
