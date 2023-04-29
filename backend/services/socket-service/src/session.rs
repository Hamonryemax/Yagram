use crate::server::WsServer;
use actix::{fut, prelude::*};
use actix_broker::BrokerIssue;
use actix_web_actors::ws;
use kafka_messages::{ClientUserMessageData, KafkaMessage, StatusMessage, UserMessage};

pub struct WsSession {}

impl WsSession {
    fn process_text_message(&self, message: &str) {
        if let Ok(message_info) = serde_json::from_str::<ClientUserMessageData>(message) {
            self.issue_system_async(UserMessage::from_client("1".to_string(), message_info));
        }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.issue_system_async(StatusMessage::connect("1".to_string()));
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.issue_system_async(StatusMessage::disconnect("1".to_string()));
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Text(text) => {
                self.process_text_message(text.trim());
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}
