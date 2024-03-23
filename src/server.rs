use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use ulid::Ulid;

use crate::auth::{get_action, Action};
use crate::message;

#[derive(Debug)]
pub struct Server {
    sessions: HashMap<Ulid, Recipient<message::Message>>,
    channels: HashMap<String, HashSet<Ulid>>,
    jwt_public_key: Vec<u8>,
}

impl Server {
    pub fn new(jwt_public_key: &[u8]) -> Server {
        Server {
            sessions: HashMap::new(),
            channels: HashMap::new(),
            jwt_public_key: jwt_public_key.to_vec(),
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

        match get_action(&msg.token, &self.jwt_public_key) {
            Ok(Action::Broadcast(channel)) => {
                if channel == msg.channel {
                    log::debug!("Broadcasting message to channel: {}", msg.channel);
                    self.broadcast(&msg.channel, &msg.msg);
                } else {
                    log::error!(
                        "Not allowed to broadcast message to channel: {}",
                        msg.channel
                    );
                }
            }
            _ => {
                log::error!("Not allowed to broadcast message");
            }
        }
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

        match get_action(&msg.token, &self.jwt_public_key) {
            Ok(Action::Join(channel)) => {
                if channel == msg.channel {
                    log::debug!("{:?} is allowed to join channel {}", msg.id, msg.channel);
                    self.channels
                        .entry(msg.channel.clone())
                        .or_default()
                        .insert(msg.id);
                } else {
                    log::error!(
                        "{:?} is not allowed to join channel {}",
                        msg.id,
                        msg.channel
                    );
                }
            }
            _ => {
                log::error!(
                    "{:?} is not allowed to join channel {}",
                    msg.id,
                    msg.channel
                );
            }
        }
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
