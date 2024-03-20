use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use ulid::Ulid;

use crate::server;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug)]
pub struct WSSession {
    pub id: Ulid,
    pub heartbeat: Instant,
    pub channels: Vec<String>,
    pub addr: Addr<server::Server>,
}

impl WSSession {
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                log::debug!("Websocket client heartbeat failed, disconnecting!");

                act.addr.do_send(server::Disconnect { id: act.id });

                ctx.stop();

                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WSSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::debug!("Websocket session started");
        self.heartbeat(ctx);

        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
                id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<server::Message> for WSSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        log::debug!("Websocket message: {:?}", msg);

        match msg {
            ws::Message::Ping(msg) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.heartbeat = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                log::debug!("Text message from websocket: {}", m);

                self.addr.do_send(server::ClientMessage {
                    id: self.id,
                    msg: m.to_string(),
                    channel: "public".to_string(),
                })
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Binary(_) => (),
            ws::Message::Nop => (),
        }
    }
}
