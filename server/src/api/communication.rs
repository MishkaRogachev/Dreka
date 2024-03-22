use actix_web::{get, post, put, delete, web, Responder, HttpResponse};

use crate::models::{communication::{LinkId, LinkDescription}, events::ClentEvent};
use super::context::ApiContext;

#[get("/comm/links/description/{link_id}")]
pub async fn get_description(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let link_id: LinkId = path.into_inner();
    let result = context.registry.communication.link(&link_id).await;

    match result {
        Ok(link) => return HttpResponse::Ok().json(link),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/comm/links/descriptions")]
pub async fn get_descriptions(context: web::Data<ApiContext>) -> impl Responder {
    let result = context.registry.communication.all_links().await;

    match result {
        Ok(links) => return HttpResponse::Ok().json(links),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/comm/links/status/{link_id}")]
pub async fn get_status(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let link_id: LinkId = path.into_inner();
    let result = context.registry.communication.status(&link_id).await;

    match result {
        Ok(status) => {
            return HttpResponse::Ok().json(status);
        },
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            return HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/comm/links/statuses")]
pub async fn get_statuses(context: web::Data<ApiContext>) -> impl Responder {
    let result = context.registry.communication.all_statuses().await;

    match result {
        Ok(statuses) => return HttpResponse::Ok().json(statuses),
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[post("/comm/links/save")]
pub async fn post_link(context: web::Data<ApiContext>, link: web::Json<LinkDescription>) -> impl Responder {
    let link = link.into_inner();
    let result = context.registry.communication.save_link(&link).await;

    match result {
        Ok(link) => {
            HttpResponse::Ok().json(link)
        },
        Err(err) => {
            log::warn!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[delete("/comm/links/remove/{link_id}")]
pub async fn delete_link(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let link_id: LinkId = path.into_inner();
    let result = context.registry.communication.delete_link(&link_id).await;

    if let Err(err) = result {
        log::warn!("REST error: {}", &err); // TODO: add path here
        return HttpResponse::InternalServerError().json(err.to_string())
    }
    HttpResponse::Ok().json(link_id)
}

#[put("/comm/links/set_connected/{link_id}")]
pub async fn set_link_connected(context: web::Data<ApiContext>, path: web::Path<String>, enabled: web::Json<bool>) -> impl Responder {
    let link_id: LinkId = path.into_inner();
    let connected = enabled.into_inner();

    match context.client_events_tx.send(ClentEvent::SetLinkEnabled { link_id: link_id.to_owned(), connected }) {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}
