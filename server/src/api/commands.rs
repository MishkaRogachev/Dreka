use actix_web::{get, post, put, web, Responder, HttpResponse};

use crate::models::{commands::*, events::ClientEvent};
use super::context::ApiContext;

#[post("/commands/execute/")]
pub async fn execute_command(context: web::Data<ApiContext>, request: web::Json<ExecuteCommandRequest>) -> impl Responder {
    let request = request.into_inner();
    let command_id: CommandId = uuid::Uuid::new_v4().to_string();

    match context.client_bus.publish(ClientEvent::ExecuteCommand { request, command_id: command_id.clone() } ) {
        Ok(_) => HttpResponse::Ok().json(command_id),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[put("/commands/cancel/")]
pub async fn cancel_command(context: web::Data<ApiContext>, request: web::Json<CommandId>) -> impl Responder {
    let command_id = request.into_inner();

    match context.client_bus.publish(ClientEvent::CancelCommand { command_id: command_id.clone() } ) {
        Ok(_) => HttpResponse::Ok().json(command_id),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/commands/execution/{command_id}")]
pub async fn get_command_execution(context: web::Data<ApiContext>, path: web::Path<String>) -> impl Responder {
    let command_id = &path.into_inner();
    let result = context.registry.commands.get_execution(command_id).await;

    match result {
        Ok(execution) => return HttpResponse::Ok().json(execution),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/commands/executions")]
pub async fn get_command_executions(context: web::Data<ApiContext>) -> impl Responder {
    let result = context.registry.commands.get_all_executions().await;

    match result {
        Ok(executions) => return HttpResponse::Ok().json(executions),
        Err(err) => {
            log::warn!("REST: error {}", &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}
