mod redis;
mod model;
mod utils;

use std::time::{Duration, Instant};

use actix::{Actor, StreamHandler, ActorContext, AsyncContext};
use actix_web::{HttpServer, App, HttpRequest, HttpResponse, web, Error};
use actix_web_actors::ws;
use uuid::Uuid;
use bytes::Bytes;

use model::{Payload, OpCode, CloseCodes, commands};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Entry point for our route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<String>,
) -> Result<HttpResponse, Error> {
    // starting the ws and creating the session with default values
    ws::start(
        Session {
            hb: Instant::now(),
            id: Uuid::new_v4(),
            ready: false
        },
        &req,
        stream
    )
}

/// The Session object describing a session of the connected user
#[derive(Clone, Debug)]
struct Session {
    hb: Instant,
    id: Uuid,
    ready: bool
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // starting heartbeat
        self.hb(ctx);

        let op = serde_json::to_value(&commands::Hello {
            heartbeat: HEARTBEAT_INTERVAL.as_secs(),
            session_id: self.id.clone()
        });

        if op.is_err() {
            ctx.stop();
            return;
        }

        let payload = serde_json::to_string(
            &Payload::from_op(
                op.unwrap(),
                OpCode::Hello,
            )
        );

        if payload.is_err() {
            ctx.stop();
            return;
        }

        // sending hello
        ctx.text(payload.unwrap());

        // check in 10 seconds if they identified or if we should close
        ctx.run_later(Duration::from_secs(10), |act, ctx| {
            if !act.ready {
                ctx.close(Some(CloseCodes::IdentifyTimeout.into()))
            }
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {

    /// handle incoming message
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            // received ping, should set hb and reply with pong
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            // received pong, should set hb
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            // received Nop, ignore
            ws::Message::Nop => (),
            // received text, handle the message
            ws::Message::Text(ref msg) => {
                match serde_json::from_str(msg) {
                    // we got a valid payload object, handle it
                    Ok(p) => self.handle_payload(ctx, p),
                    // we got an invalid packet, close the stream
                    Err(_) => ctx.close(Some(CloseCodes::InvalidPacket.into()))
                }
            },
            // we got an invalid packet, close the stream
            _ => {
                ctx.close(Some(CloseCodes::InvalidPacket.into()))
            }
        }
    }
}

impl Session {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");
                // stop actor
                ctx.close(Some(CloseCodes::HeartbeatTimeout.into()));
                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    fn handle_payload(&mut self, ctx: &mut ws::WebsocketContext<Self>, payload: Payload) {
        match payload.op {
            OpCode::Identify => {}
            _ => ctx.close(Some(CloseCodes::InvalidOpCode.into()))
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let url = std::env::var("REDIS_URL").expect("no REDIS_URL has been set");
    let redis_client = redis::RedisClient::new(url).map_err(
        |e| std::io::Error::new(std::io::ErrorKind::Other, e)
    )?;

    HttpServer::new(move || {
        App::new()
            .data(redis_client.clone())
            .service(web::resource("/ws/").to(chat_route))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
