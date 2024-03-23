use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use ulid::Ulid;

use crate::message;

#[derive(Debug)]
pub struct Server {
    sessions: HashMap<Ulid, Recipient<message::Message>>,
    channels: HashMap<String, HashSet<Ulid>>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            sessions: HashMap::new(),
            channels: HashMap::new(),
        }
    }

    fn broadcast(&self, channel: &str, message: &str) {
        log::debug!("Broadcasting message to channel: {}: {}", channel, message);
        if let Some(sessions) = self.channels.get(channel) {
            for id in sessions {
                if let Some(addr) = self.sessions.get(id) {
                    addr.do_send(message::Message(message.to_owned()));
                }
            }
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<message::Broadcast> for Server {
    type Result = ();
    fn handle(&mut self, msg: message::Broadcast, _: &mut Context<Self>) {
        log::debug!("handling Broadcast: {:?}", msg);
        self.broadcast(&msg.channel, &msg.msg);
    }
}

impl Handler<message::Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: message::Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id, msg.addr);
    }
}

impl Handler<message::Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: message::Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<message::JoinChannel> for Server {
    type Result = ();

    fn handle(&mut self, msg: message::JoinChannel, _: &mut Context<Self>) {
        log::debug!("{:?} joining channel {}", msg.id, msg.channel);

        self.channels
            .entry(msg.channel.clone())
            .or_default()
            .insert(msg.id);
    }
}

impl Handler<message::LeaveChannel> for Server {
    type Result = ();

    fn handle(&mut self, msg: message::LeaveChannel, _: &mut Context<Self>) {
        log::debug!("{:?} leaving channel {}", msg.id, msg.channel);

        if let Some(channel) = self.channels.get_mut(&msg.channel) {
            channel.remove(&msg.id);
        }
    }
}

impl Handler<message::LeaveAllChannels> for Server {
    type Result = ();

    fn handle(&mut self, msg: message::LeaveAllChannels, _: &mut Context<Self>) {
        log::debug!("{:?} leaving all channels", msg.id);

        for (_, channel) in self.channels.iter_mut() {
            channel.remove(&msg.id);
        }
    }
}
