use std::collections::{HashMap, HashSet};
use ulid::Ulid;

use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub id: Ulid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Ulid,
}

#[derive(Debug)]
pub struct Server {
    sessions: HashMap<Ulid, Recipient<Message>>,
    channels: HashMap<String, HashSet<Ulid>>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: Ulid,
    pub msg: String,
    pub channel: String,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Broadcast {
    pub msg: String,
    pub channel: String,
}

impl Server {
    pub fn new() -> Server {
        let mut channels = HashMap::new();
        channels.insert("public".to_string(), HashSet::new());

        Server {
            sessions: HashMap::new(),
            channels,
        }
    }

    fn broadcast(&self, channel: &str, message: &str) {
        log::debug!("Broadcasting message to channel: {}: {}", channel, message);
        if let Some(sessions) = self.channels.get(channel) {
            for id in sessions {
                if let Some(addr) = self.sessions.get(id) {
                    addr.do_send(Message(message.to_owned()));
                }
            }
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<ClientMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        log::debug!("handling ClientMessage: {:?}", msg);
        self.broadcast(&msg.channel, &msg.msg);
    }
}

impl Handler<Broadcast> for Server {
    type Result = ();
    fn handle(&mut self, msg: Broadcast, _: &mut Context<Self>) {
        log::debug!("handling Broadcast: {:?}", msg);
        self.broadcast(&msg.channel, &msg.msg);
    }
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id, msg.addr);

        // Everyone joins public by default
        self.channels
            .entry("public".to_string())
            .or_default()
            .insert(msg.id);
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}
