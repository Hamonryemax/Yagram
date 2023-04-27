use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
enum StatusMessageType {
    #[serde(rename = "connect")]
    Connect,
    #[serde(rename = "disconnect")]
    Disconnect,
}

#[derive(Clone, Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct StatusMessage {
    status: StatusMessageType,
    user_id: String,
}

impl StatusMessage {
    pub fn connect(user_id: String) -> Self {
        StatusMessage {
            status: StatusMessageType::Connect,
            user_id,
        }
    }
    pub fn disconnect(user_id: String) -> Self {
        StatusMessage {
            status: StatusMessageType::Disconnect,
            user_id,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClientTextMessageData {
    receiver_id: String,
    text: String,
}

#[derive(Clone, Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct TextMessage {
    user_id: String,
    receiver_id: String,
    text: String,
}

impl TextMessage {
    pub fn from_client(user_id: String, data: ClientTextMessageData) -> Self {
        TextMessage {
            user_id,
            receiver_id: data.receiver_id,
            text: data.text,
        }
    }
}
