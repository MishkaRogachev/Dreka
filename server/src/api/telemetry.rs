// use actix_web::{get, web, Responder, HttpResponse};

// use crate::models::telemetry::{FlightData, SnsData, SensorsData};
// use crate::context::Context;

// // FIXME: replace REST with RPC JSon for telemetry
// #[get("/telemetry/flight/{vehicle_id}")]
// pub async fn get_flight_data(context: web::Data<Context>, path: web::Path<String>) -> impl Responder {
//     let id = &path.into_inner();
//     let result = context.repository.read::<FlightData>("vehicle_flight_data", id).await;

//     match result {
//         Ok(vehicle) => return HttpResponse::Ok().json(vehicle),
//         Err(err) => {
//             log::warn!("REST(/telemetry/flight/{}): error {}", &id, &err);
//             HttpResponse::InternalServerError().json(err.to_string())
//         }
//     }
// }

// #[get("/telemetry/sns/{vehicle_id}")]
// pub async fn get_sns_data(context: web::Data<Context>, path: web::Path<String>) -> impl Responder {
//     let id = &path.into_inner();
//     let result = context.repository.read::<SnsData>("vehicle_sns_data", id).await;

//     match result {
//         Ok(vehicle) => return HttpResponse::Ok().json(vehicle),
//         Err(err) => {
//             log::warn!("REST(/telemetry/sns/{}): error {}", &id, &err);
//             HttpResponse::InternalServerError().json(err.to_string())
//         }
//     }
// }

// #[get("/telemetry/sensors/{vehicle_id}")]
// pub async fn get_sensors_data(context: web::Data<Context>, path: web::Path<String>) -> impl Responder {
//     let id = &path.into_inner();
//     let result = context.repository.read::<SensorsData>("vehicle_sensors_data", id).await;

//     match result {
//         Ok(vehicle) => return HttpResponse::Ok().json(vehicle),
//         Err(err) => {
//             log::warn!("REST(/telemetry/sensors/{}): error {}", &id, &err);
//             HttpResponse::InternalServerError().json(err.to_string())
//         }
//     }
// }
