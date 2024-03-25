use actix_web::{get, post, delete, web, Responder, HttpResponse};

use crate::models::commands::VehicleCommand;
use super::context::ApiContext;

#[post("/commands/exec/{vehicle_id}")]
pub async fn execute_command(context: web::Data<ApiContext>, path: web::Path<String>, command: web::Json<VehicleCommand>) -> impl Responder {
    let command = command.into_inner();
    let vehicle_id = &path.into_inner();

    let result = context.registry.commands.register_vehicle_command(&vehicle_id, &command).await;
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

#[get("/commands/{command_id}")]
pub async fn get_command(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let command_id = &path.into_inner();
    let result = context.registry.commands.get_vehicle_command(command_id).await;

    match result {
        Ok(command) => return HttpResponse::Ok().json(command),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/commands")]
pub async fn get_commands(context: web::Data<ApiContext>) -> impl Responder {
    let result = context.registry.commands.all_vehicle_commands().await;

    match result {
        Ok(commands) => return HttpResponse::Ok().json(commands),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[delete("/commands/cancel/{command_id}")]
pub async fn cancel_command(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let command_id = &path.into_inner();
    let result = context.registry.commands.drop_vehicle_command(command_id).await;

    match result {
        Ok(_) => return HttpResponse::Ok().json(command_id),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}
