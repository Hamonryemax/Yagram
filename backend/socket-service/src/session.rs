use crate::messages::StatusMessage;
use crate::server::WsServer;
use actix::{fut, prelude::*};
use actix_broker::BrokerIssue;
use actix_web_actors::ws;

pub struct WsSession {}

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
                println!("{}", text)
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}
