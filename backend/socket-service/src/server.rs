use crate::messages::StatusMessage;
use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use serde_json::json;

use kafka_client::MessageProducer;

pub struct WsServer {
    message_producer: MessageProducer,
}

impl Default for WsServer {
    fn default() -> Self {
        WsServer {
            message_producer: MessageProducer::new("localhost:9093".to_string()),
        }
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<StatusMessage>(ctx);
    }
}

impl Handler<StatusMessage> for WsServer {
    type Result = ();
    fn handle(&mut self, msg: StatusMessage, ctx: &mut Self::Context) -> Self::Result {
        let fut = Box::pin(async move {
            let message = json!(msg);
            let result = MessageProducer::new("localhost:9093".to_string())
                .produce("messages", message.to_string())
                .await;
            println!("result: {:?}", result);
        });
        let fut_actor = fut.into_actor(self);
        ctx.spawn(fut_actor);
    }
}

impl SystemService for WsServer {}
impl Supervised for WsServer {}
