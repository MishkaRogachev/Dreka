// use actix_web::{get, post, delete, web, Responder, HttpResponse};

// use crate::models::commands::{Command, State, Execution};
// use crate::context::Context;

// #[post("/commands/exec/{vehicle_id}")]
// pub async fn execute_command(context: web::Data<Context>, command: web::Json<Command>, path: web::Path<String>) -> impl Responder {
//     let command = command.into_inner();
//     let vehicle_id = &path.into_inner();
//     let execution = Execution::create(command, vehicle_id);

//     let result = context.repository.upsert("vehicle_commands", &execution).await;

//     match result {
//         Ok(execution) => {
//             HttpResponse::Ok().json(execution)
//         },
//         Err(err) => {
//             log::warn!("REST(/commands/exec/{}): error {}", &vehicle_id, &err);
//             HttpResponse::InternalServerError().json(err.to_string())
//         }
//     }
// }

// #[get("/commands/get/{command_id}")]
// pub async fn get_command(context: web::Data<Context>, path: web::Path<String>) -> impl Responder {
//     let command_id = &path.into_inner();
//     let result = context.repository.read::<Execution>("vehicle_commands", command_id).await;

//     match result {
//         Ok(command) => return HttpResponse::Ok().json(command),
//         Err(err) => {
//             log::warn!("REST(/commands/get/{}): error {}", &command_id, &err);
//             HttpResponse::InternalServerError().json(err.to_string())
//         }
//     }
// }

// #[delete("/commands/cancel/{command_id}")]
// pub async fn cancel_command(context: web::Data<Context>, path: web::Path<String>) -> impl Responder {
//     let command_id = &path.into_inner();
//     let result = context.repository.read::<Execution>("vehicle_commands", command_id).await;

//     match result {
//         Ok(mut execution) => {
//             execution.state = State::Canceled;
//             let result = context.repository.upsert("vehicle_commands", &execution).await;
//             match result {
//                 Ok(execution) => {
//                     HttpResponse::Ok().json(execution)
//                 },
//                 Err(err) => {
//                     log::warn!("REST(/commands/exec/{}): error {}", &command_id, &err);
//                     HttpResponse::InternalServerError().json(err.to_string())
//                 }
//             }
//         },
//         Err(err) => {
//             log::warn!("REST(/commands/stop/{}): error {}", &command_id, &err);
//             HttpResponse::InternalServerError().json(err.to_string())
//         }
//     }
// }
