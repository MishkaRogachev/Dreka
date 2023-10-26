use actix_web::{get, post, put, delete, web, Responder, HttpResponse};

use crate::models::{communication::{LinkDescription, LinkStatus}, events::ClentEvent};
use super::shared::Shared;

#[get("/comm/links")]
pub async fn list_descriptions(shared: web::Data<Shared>) -> impl Responder {
    let result = shared.repository.read_all::<LinkDescription>("link_descriptions").await;

    match result {
        Ok(links) => return HttpResponse::Ok().json(links),
        Err(err) => {
            println!("REST error: {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/comm/link/{link_id}")]
pub async fn link_description(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let id = &path.into_inner();
    let result = shared.repository.read::<LinkDescription>("link_descriptions", id).await;

    match result {
        Ok(links) => return HttpResponse::Ok().json(links),
        Err(err) => {
            println!("REST error: {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[post("/comm/links/save")]
pub async fn save_description(shared: web::Data<Shared>, link: web::Json<LinkDescription>) -> impl Responder {
    let link = link.into_inner();
    let result = shared.repository.upsert("link_descriptions", &link).await;

    match result {
        Ok(link) => {
            HttpResponse::Ok().json(link)
        },
        Err(err) => {
            println!("REST error: {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[delete("/comm/links/remove/{link_id}")]
pub async fn remove_description(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let link_id = &path.into_inner();
    let result = shared.repository.remove("link_descriptions", &link_id).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(link_id),
        Err(err) => {
            println!("REST error: {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/comm/links/status/{link_id}")]
pub async fn get_status(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let link_id: String = path.into_inner();
    let result = shared.repository.read::<LinkStatus>("link_statuses", &link_id).await;

    match result {
        Ok(link_status) => {
            return HttpResponse::Ok().json(link_status);
        },
        Err(err) => {
            if let crate::datasource::db::DbError::NoData = err {
                return HttpResponse::Ok().json(LinkStatus::default_for_id(&link_id))
            }
            println!("REST error: {}", &err);
            return HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[put("/comm/links/set_connected/{link_id}")]
pub async fn set_link_enabled(shared: web::Data<Shared>, path: web::Path<String>, enabled: web::Json<bool>) -> impl Responder {
    let id = &path.into_inner();
    let connected = enabled.into_inner();

    match shared.tx.send(ClentEvent::SetLinkConnected { link_id: id.to_owned(), connected }) {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}
