use actix_web::{get, post, delete, web, Responder, HttpResponse};

use crate::models::commands::{Command, State, Execution};
use super::shared::Shared;

#[post("/commands/exec/{vehicle_id}")]
pub async fn execute_command(shared: web::Data<Shared>, command: web::Json<Command>, path: web::Path<String>) -> impl Responder {
    let command = command.into_inner();
    let vehicle_id = &path.into_inner();
    let execution = Execution::create(command, vehicle_id);

    let result = shared.repository.upsert("vehicle_commands", &execution).await;

    match result {
        Ok(execution) => {
            HttpResponse::Ok().json(execution)
        },
        Err(err) => {
            println!("REST(/commands/exec/{}): error {}", &vehicle_id, &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[get("/commands/get/{command_id}")]
pub async fn get_command(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let command_id = &path.into_inner();
    let result = shared.repository.read::<Execution>("vehicle_commands", command_id).await;

    match result {
        Ok(command) => return HttpResponse::Ok().json(command),
        Err(err) => {
            println!("REST(/commands/get/{}): error {}", &command_id, &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}

#[delete("/commands/cancel/{command_id}")]
pub async fn cancel_command(shared: web::Data<Shared>, path: web::Path<String>) -> impl Responder {
    let command_id = &path.into_inner();
    let result = shared.repository.read::<Execution>("vehicle_commands", command_id).await;

    match result {
        Ok(mut execution) => {
            execution.state = State::Canceled;
            let result = shared.repository.upsert("vehicle_commands", &execution).await;
            match result {
                Ok(execution) => {
                    HttpResponse::Ok().json(execution)
                },
                Err(err) => {
                    println!("REST(/commands/exec/{}): error {}", &command_id, &err);
                    HttpResponse::InternalServerError().json(err.to_string())
                }
            }
        },
        Err(err) => {
            println!("REST(/commands/stop/{}): error {}", &command_id, &err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}
