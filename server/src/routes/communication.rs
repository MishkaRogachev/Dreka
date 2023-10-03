use std::str::FromStr;
use std::sync::Arc;
use actix_web::{get, post, delete, web, Responder, HttpResponse};

use crate::{db::persistence, models::communication::{LinkDescription, LinkStatus}};

#[get("/comm/links")]
pub async fn list_descriptions(persistence: web::Data<Arc<persistence::Persistence>>) -> impl Responder {
    let response = persistence.read_all::<LinkDescription>("links").await;
    match response {
        Ok(links) => return HttpResponse::Ok().json(links),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/comm/links/create")]
pub async fn add_description(persistence: web::Data<Arc<persistence::Persistence>>, new_link: web::Json<LinkDescription>) -> impl Responder {
    let result = persistence.create("links", &new_link.into_inner()).await;
    match result {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/comm/links/update")]
pub async fn update_description(persistence: web::Data<Arc<persistence::Persistence>>, link: web::Json<LinkDescription>) -> impl Responder {
    let result = persistence.update("links", &link.into_inner()).await;
    match result {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/comm/links/remove/{link_id}")]
pub async fn remove_description(persistence: web::Data<Arc<persistence::Persistence>>, path: web::Path<String>) -> impl Responder {
    let link_id = surrealdb::sql::Thing::from_str(&path.into_inner());
    if let Err(_) = link_id {
        return HttpResponse::InternalServerError().body("Error converting to internal id");
    }

    let result = persistence.remove::<LinkDescription>("links", link_id.unwrap()).await;
    match result {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/comm/links/status/{link_id}")]
pub async fn get_status(persistence: web::Data<Arc<persistence::Persistence>>, path: web::Path<String>) -> impl Responder {
    let link_id = surrealdb::sql::Thing::from_str(&path.into_inner());
    if let Err(_) = link_id {
        return HttpResponse::InternalServerError().body("Error converting to internal id");
    }

    let response = persistence.read::<LinkStatus>("links", link_id.unwrap()).await;
    match response {
        Ok(link_status) => return HttpResponse::Ok().json(link_status),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
