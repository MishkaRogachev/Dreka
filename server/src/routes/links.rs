use crate::{db::persistence, models::links::LinkDescription};

use std::sync::Arc;
use actix_web::{get, post, web, Responder, HttpResponse};

#[get("/links")]
pub async fn list_descriptions(persistence: web::Data<Arc<persistence::Persistence>>) -> impl Responder {
    let response = persistence.read_all::<LinkDescription>("links").await;
    match response {
        Ok(links) => return HttpResponse::Ok().json(links),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/links/create")]
pub async fn add_description(persistence: web::Data<Arc<persistence::Persistence>>, new_link: web::Json<LinkDescription>) -> impl Responder {
    let result = persistence.create("links", &new_link.into_inner()).await;
    match result {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/links/update")]
pub async fn update_description(persistence: web::Data<Arc<persistence::Persistence>>, link: web::Json<LinkDescription>) -> impl Responder {
    let result = persistence.update("links", &link.into_inner()).await;
    match result {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/links/remove")]
pub async fn remove_description(persistence: web::Data<Arc<persistence::Persistence>>, link_id: web::Json<String>) -> impl Responder {
    let result = persistence.remove::<LinkDescription>("links", surrealdb::sql::Id::from(&link_id.into_inner())).await;
    match result {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
