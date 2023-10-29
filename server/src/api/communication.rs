use actix_web::{get, post, put, delete, web, Responder, HttpResponse};

use crate::{models::{communication::{LinkDescription, LinkStatus}, events::ClentEvent}, datasource::db::DbError};
use super::shared::Shared;

#[get("/comm/links/description/{link_id}")]
pub async fn get_description(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let id = &path.into_inner();
    let result = shared.repository.read::<LinkDescription>("link_descriptions", id).await;

    match result {
        Ok(link) => return HttpResponse::Ok().json(link),
        Err(err) => {
            println!("REST error: {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/comm/links/descriptions")]
pub async fn get_descriptions(shared: web::Data<Shared>) -> impl Responder {
    let result = shared.repository.read_all::<LinkDescription>("link_descriptions").await;

    match result {
        Ok(links) => return HttpResponse::Ok().json(links),
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
        Ok(status) => {
            return HttpResponse::Ok().json(status);
        },
        Err(err) => {
            if let crate::datasource::db::DbError::NoData = err {
                return HttpResponse::Ok().json(LinkStatus::default_for_id(&link_id))
            }
            return HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/comm/links/statuses")]
pub async fn get_statuses(shared: web::Data<Shared>) -> impl Responder {
    let result = shared.repository.read_all::<LinkStatus>("link_statuses").await;

    match result {
        Ok(statuses) => return HttpResponse::Ok().json(statuses),
        Err(err) => {
            println!("REST error: {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[post("/comm/links/save")]
pub async fn post_link(shared: web::Data<Shared>, link: web::Json<LinkDescription>) -> impl Responder {
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
pub async fn delete_link(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let link_id = &path.into_inner();

    let result = shared.repository.remove("link_descriptions", &link_id).await;
    if let Err(err) = result {
        println!("REST error: {}", &err);
        return HttpResponse::InternalServerError().json(err.to_string())
    }

    let result = shared.repository.remove("link_statuses", &link_id).await;
    if let Err(err) = result {
        if let DbError::NoData = err {
            println!("REST error: {}", &err);
            return HttpResponse::InternalServerError().json(err.to_string())
        }
    }

    HttpResponse::Ok().json(link_id)
}

#[put("/comm/links/set_connected/{link_id}")]
pub async fn set_link_connected(shared: web::Data<Shared>, path: web::Path<String>, enabled: web::Json<bool>) -> impl Responder {
    let id = &path.into_inner();
    let connected = enabled.into_inner();

    match shared.tx.send(ClentEvent::SetLinkConnected { link_id: id.to_owned(), connected }) {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}
