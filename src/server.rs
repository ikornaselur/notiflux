use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use ulid::Ulid;

use crate::auth::{get_action, Action};
use crate::message;

#[derive(Debug)]
pub struct Server {
    sessions: HashMap<Ulid, Recipient<message::Message>>,
    topics: HashMap<String, HashSet<Ulid>>,
    jwt_public_key: Vec<u8>,
}

impl Server {
    pub fn new(jwt_public_key: &[u8]) -> Server {
        Server {
            sessions: HashMap::new(),
            topics: HashMap::new(),
            jwt_public_key: jwt_public_key.to_vec(),
        }
    }

    fn broadcast(&self, topic: &str, message: &str) {
        log::debug!("Broadcasting message to topic: {}: {}", topic, message);
        if let Some(sessions) = self.topics.get(topic) {
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
            Ok(Action::Broadcast(topic)) => {
                if topic == msg.topic {
                    log::debug!("Broadcasting message to topic: {}", msg.topic);
                    self.broadcast(&msg.topic, &msg.msg);
                } else {
                    log::error!("Not allowed to broadcast message to topic: {}", msg.topic);
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

impl Handler<message::SubscribeToTopic> for Server {
    type Result = ();

    fn handle(&mut self, msg: message::SubscribeToTopic, _: &mut Context<Self>) {
        log::debug!("{:?} subscribing topic {}", msg.id, msg.topic);

        match get_action(&msg.token, &self.jwt_public_key) {
            Ok(Action::Subscribe(topic)) => {
                if topic == msg.topic {
                    log::debug!("{:?} is allowed to subscribe topic {}", msg.id, msg.topic);
                    self.topics
                        .entry(msg.topic.clone())
                        .or_default()
                        .insert(msg.id);
                } else {
                    log::error!(
                        "{:?} is not allowed to subscribe topic {}",
                        msg.id,
                        msg.topic
                    );
                }
            }
            _ => {
                log::error!(
                    "{:?} is not allowed to subscribe topic {}",
                    msg.id,
                    msg.topic
                );
            }
        }
    }
}

impl Handler<message::UnsubscribeFromTopic> for Server {
    type Result = ();

    fn handle(&mut self, msg: message::UnsubscribeFromTopic, _: &mut Context<Self>) {
        log::debug!("{:?} leaving topic {}", msg.id, msg.topic);

        if let Some(topic) = self.topics.get_mut(&msg.topic) {
            topic.remove(&msg.id);
        }
    }
}

impl Handler<message::UnsubscribeAll> for Server {
    type Result = ();

    fn handle(&mut self, msg: message::UnsubscribeAll, _: &mut Context<Self>) {
        log::debug!("{:?} leaving all topics", msg.id);

        for (_, topic) in self.topics.iter_mut() {
            topic.remove(&msg.id);
        }
    }
}
