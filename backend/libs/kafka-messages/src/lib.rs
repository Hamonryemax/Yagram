use serde::{Deserialize, Serialize};

mod status_message;
mod user_message;

pub use status_message::*;
pub use user_message::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "message_type", content = "data")]
pub enum KafkaMessage {
    StatusMessage(StatusMessage),
    UserMessage(UserMessage),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;

    #[test]
    fn kafka_connect_message() {
        let msg = StatusMessage::connect("1".to_string());
        let kafka_msg = KafkaMessage::from(msg);
        let json_value = json!(kafka_msg).to_string();
        assert_eq!(
            json_value, "{\"data\":{\"status\":\"connect\",\"user_id\":\"1\"},\"message_type\":\"StatusMessage\"}",
            "Serialized value = {}",
            json_value
        );
    }

    #[test]
    fn kafka_disconnect_message() {
        let msg = StatusMessage::disconnect("1".to_string());
        let kafka_msg = KafkaMessage::from(msg);
        let json_value = json!(kafka_msg).to_string();
        assert_eq!(
            json_value, "{\"data\":{\"status\":\"disconnect\",\"user_id\":\"1\"},\"message_type\":\"StatusMessage\"}",
            "Serialized value = {}",
            json_value
        );
    }

    #[test]
    fn kafka_user_message() {
        let msg = UserMessage::from_client(
            "1".to_string(),
            ClientUserMessageData {
                receiver_id: "2".to_string(),
                text: "Hello world".to_string(),
            },
        );
        let kafka_msg = KafkaMessage::from(msg);
        let json_value = json!(kafka_msg).to_string();
        assert_eq!(
            json_value, "{\"data\":{\"receiver_id\":\"2\",\"text\":\"Hello world\",\"user_id\":\"1\"},\"message_type\":\"UserMessage\"}",
            "Serialized value = {}",
            json_value
        );
    }

    #[test]
    fn deserialize_connect_message() {
        let json = "{\"data\":{\"status\":\"connect\",\"user_id\":\"1\"},\"message_type\":\"StatusMessage\"}";
        let message_result = serde_json::from_str::<KafkaMessage>(json);
        assert!(
            message_result.is_ok(),
            "Parse result = {:?}",
            message_result
        );
    }
}
