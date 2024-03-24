use std::time::Duration;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use actix::AsyncContext;
use flume::Receiver;

use crate::models::telemetry::VehicleTelemetry;

use super::context::ApiContext;

const POLL_INTERVAL: Duration = Duration::from_millis(5);

pub struct WebSocketActor {
    telemetry_rx: Receiver<VehicleTelemetry>
}

impl WebSocketActor {
    pub fn new(telemetry_rx: Receiver<VehicleTelemetry>) -> Self {
        Self { telemetry_rx }
    }
}

impl actix::Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Starting the telemetry websocket connection..");

        ctx.run_interval(POLL_INTERVAL, |myself, ctx| {
            if let Ok(telemetry) = myself.telemetry_rx.try_recv() {
                match serde_json::to_string(&telemetry) {
                    Ok(json) => ctx.text(json),
                    Err(err) => log::warn!("Failed to serialize telemetry: {:?}", err),
                };
            }
        });
    }
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => {},
            Ok(ws::Message::Text(_)) => { /* Handle text message */ },
            Ok(ws::Message::Close(reason)) => {
                log::info!("Closing the telemetry websocket connection..");
                ctx.close(reason)
            },
            _ => {}
        }
    }
}

#[get("/telemetry/ws")]
pub async fn telemetry_ws(context: web::Data<ApiContext>, req: HttpRequest, stream: web::Payload) -> impl Responder {
    log::info!("Start..");
    let actor = WebSocketActor::new(context.telemetry_rx.clone());
    match ws::start(actor, &req, stream) {
        Ok(res) => {
            return res;
        },
        Err(err) => {
            log::warn!("Failed to start websocket: {:?}", err);
            HttpResponse::InternalServerError().json(err.to_string())
        }
    }
}
