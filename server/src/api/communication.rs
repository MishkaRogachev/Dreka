use actix_web::{get, post, put, delete, web, Responder, HttpResponse};

use crate::models::{communication::{LinkId, LinkDescription}, events::ClentEvent};
use crate::context::AppContext;

#[get("/comm/links/description/{link_id}")]
pub async fn get_description(context: web::Data<AppContext>, path: web::Path<String>) -> impl Responder {
    let link_id: LinkId = path.into_inner();
    let result = context.communication.link(&link_id).await;

    match result {
        Ok(link) => return HttpResponse::Ok().json(link),
        Err(err) => {
            println!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/comm/links/descriptions")]
pub async fn get_descriptions(context: web::Data<AppContext>) -> impl Responder {
    let result = context.communication.all_links().await;

    match result {
        Ok(links) => return HttpResponse::Ok().json(links),
        Err(err) => {
            println!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/comm/links/status/{link_id}")]
pub async fn get_status(context: web::Data<AppContext>, path: web::Path<String>) -> impl Responder {
    let link_id: LinkId = path.into_inner();
    let result = context.communication.status(&link_id).await;

    match result {
        Ok(status) => {
            return HttpResponse::Ok().json(status);
        },
        Err(err) => {
            println!("REST error: {}", &err); // TODO: add path here
            return HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/comm/links/statuses")]
pub async fn get_statuses(context: web::Data<AppContext>) -> impl Responder {
    let result = context.communication.all_statuses().await;

    match result {
        Ok(statuses) => return HttpResponse::Ok().json(statuses),
        Err(err) => {
            println!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[post("/comm/links/save")]
pub async fn post_link(context: web::Data<AppContext>, link: web::Json<LinkDescription>) -> impl Responder {
    let link = link.into_inner();
    let result = context.communication.save_link(&link).await;

    match result {
        Ok(link) => {
            HttpResponse::Ok().json(link)
        },
        Err(err) => {
            println!("REST error: {}", &err); // TODO: add path here
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[delete("/comm/links/remove/{link_id}")]
pub async fn delete_link(context: web::Data<AppContext>, path: web::Path<String>) -> impl Responder {
    let link_id: LinkId = path.into_inner();
    let result = context.communication.delete_link(&link_id).await;

    if let Err(err) = result {
        println!("REST error: {}", &err); // TODO: add path here
        return HttpResponse::InternalServerError().json(err.to_string())
    }
    HttpResponse::Ok().json(link_id)
}

#[put("/comm/links/set_connected/{link_id}")]
pub async fn set_link_connected(context: web::Data<AppContext>, path: web::Path<String>, enabled: web::Json<bool>) -> impl Responder {
    let link_id: LinkId = path.into_inner();
    let connected = enabled.into_inner();

    match context.tx.send(ClentEvent::SetLinkEnabled { link_id: link_id.to_owned(), connected }) {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError()
    }
}
