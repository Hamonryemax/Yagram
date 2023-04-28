use crate::KafkaMessage;
use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
enum StatusMessageType {
    #[serde(rename = "connect")]
    Connect,
    #[serde(rename = "disconnect")]
    Disconnect,
}

#[derive(Clone, Message, Serialize, Deserialize, Debug)]
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

impl From<StatusMessage> for KafkaMessage {
    fn from(value: StatusMessage) -> Self {
        KafkaMessage::StatusMessage(value)
    }
}
