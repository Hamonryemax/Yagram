use crate::KafkaMessage;
use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Message, Serialize, Deserialize, Debug)]
#[rtype(result = "()")]
pub struct UserMessage {
    user_id: String,
    receiver_id: String,
    text: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClientUserMessageData {
    pub receiver_id: String,
    pub text: String,
}

impl UserMessage {
    pub fn from_client(user_id: String, data: ClientUserMessageData) -> Self {
        Self {
            user_id,
            receiver_id: data.receiver_id,
            text: data.text,
        }
    }

    pub fn key(&self) -> String {
        format!("for_user_{}", self.receiver_id)
    }
}

impl From<UserMessage> for KafkaMessage {
    fn from(value: UserMessage) -> Self {
        KafkaMessage::UserMessage(value)
    }
}
