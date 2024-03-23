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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_channel_validate_token() {
        let public_key = include_str!("../scripts/public_key.pem");
        let token = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJjaGFubmVsIjoiZm9vIiwiZXhwIjo0ODY0NzkyMzcyLCJzY29wZSI6ImpvaW4ifQ.c11kOY3w2YGyFHzlxMx1QR77vHxFFHhF7cWu4b4tADXHh4VINk318EiughK35QV5n7rgK45hrzkeQ_Xhg1ThyA
";

        let join = JoinChannel {
            id: Ulid::new(),
            channel: "test".to_string(),
            token: token.to_string(),
        };

        assert!(join.validate_token(public_key));
    }
}
