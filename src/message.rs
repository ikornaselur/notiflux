use actix::prelude::*;
use ulid::Ulid;

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

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinChannel {
    pub id: Ulid,
    pub channel: String,
    pub token: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveChannel {
    pub id: Ulid,
    pub channel: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveAllChannels {
    pub id: Ulid,
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
    pub token: String,
}
