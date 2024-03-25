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
pub struct SubscribeToTopic {
    pub id: Ulid,
    pub topic: String,
    pub token: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct UnsubscribeFromTopic {
    pub id: Ulid,
    pub topic: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct UnsubscribeAll {
    pub id: Ulid,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Broadcast {
    pub msg: String,
    pub topic: String,
    pub token: String,
}
