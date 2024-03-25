use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use ulid::Ulid;

use crate::{message, server};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug)]
pub struct WSSession {
    pub id: Ulid,
    pub heartbeat: Instant,
    pub topics: Vec<String>,
    pub addr: Addr<server::Server>,
}

impl WSSession {
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                log::debug!("Websocket client heartbeat failed, disconnecting!");

                act.addr.do_send(message::Disconnect { id: act.id });

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
            .send(message::Connect {
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
        self.addr.do_send(message::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<message::Message> for WSSession {
    type Result = ();

    fn handle(&mut self, msg: message::Message, ctx: &mut Self::Context) {
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

        log::trace!("Websocket message: {:?}", msg);

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

                // We only care about messages that start with a slash
                if !m.starts_with('/') {
                    return;
                }

                let args: Vec<&str> = m.splitn(2, ' ').collect();
                match args[..] {
                    ["/subscribe", sub_args] => {
                        let sub_args: Vec<&str> = sub_args.splitn(2, ' ').collect();
                        if let [topic, token] = sub_args[..] {
                            self.addr.do_send(message::SubscribeToTopic {
                                id: self.id,
                                topic: topic.to_string(),
                                token: token.to_string(),
                            });
                        } else {
                            ctx.text("Invalid subscribe command, it should be: /subscribe <topic> <token>");
                        }
                    }
                    ["/unsubscribe", topic] => {
                        self.addr.do_send(message::UnsubscribeFromTopic {
                            id: self.id,
                            topic: topic.to_string(),
                        });
                    }
                    ["/unsubscribe-all"] => {
                        self.addr.do_send(message::UnsubscribeAll { id: self.id });
                    }
                    _ => {
                        ctx.text("Unknown command");
                    }
                }
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
