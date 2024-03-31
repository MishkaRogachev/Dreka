use std::time::Duration;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use actix::AsyncContext;
use tokio::sync::broadcast::Receiver;

use crate::models::events::ServerEvent;

use super::context::ApiContext;

const POLL_INTERVAL: Duration = Duration::from_millis(5);

pub struct WebSocketActor {
    events: Receiver<ServerEvent>
}

impl WebSocketActor {
    pub fn new(events: Receiver<ServerEvent>) -> Self {
        Self { events }
    }
}

impl actix::Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Starting the websocket connection..");

        ctx.run_interval(POLL_INTERVAL, |myself, ctx| {
            if let Ok(event) = myself.events.try_recv() {
                match serde_json::to_string(&event) {
                    Ok(json) => ctx.text(json),
                    Err(err) => log::warn!("Failed to serialize event: {:?}", err),
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
                log::info!("Closing the websocket connection");
                ctx.close(reason)
            },
            _ => {}
        }
    }
}

#[get("/ws")]
pub async fn events_ws(context: web::Data<ApiContext>, req: HttpRequest, stream: web::Payload) -> impl Responder {
    let actor = WebSocketActor::new(context.server_bus.subscribe());
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
