use crate::messages::{StatusMessage, TextMessage};
use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use serde::Serialize;
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

// impl WsServer {
//     fn send_to_kafka<Message: Serialize>(&self, msg: Message, ctx: &mut Context<Self>) {
//         let mut producer = self.message_producer.clone();
//         let fut = Box::pin(async move {
//             let message = json!(msg);
//             let result = producer.produce("messages", message.to_string()).await;
//             match result {
//                 Ok(_) => {}
//                 Err(e) => println!("{:?}", e),
//             }
//         });
//         let fut_actor = fut.into_actor(self);
//         ctx.spawn(fut_actor);
//     }
// }

impl Actor for WsServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<StatusMessage>(ctx);
        self.subscribe_system_async::<TextMessage>(ctx);
    }
}

impl Handler<StatusMessage> for WsServer {
    type Result = ();
    fn handle(&mut self, msg: StatusMessage, ctx: &mut Self::Context) -> Self::Result {
        let mut producer = self.message_producer.clone();
        let fut = Box::pin(async move {
            let message = json!(msg);
            let result = producer.produce("messages", message.to_string()).await;
            match result {
                Ok(_) => {}
                Err(e) => println!("{:?}", e),
            }
        });
        let fut_actor = fut.into_actor(self);
        ctx.spawn(fut_actor);
    }
}

impl Handler<TextMessage> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) -> Self::Result {
        let mut producer = self.message_producer.clone();
        let fut = Box::pin(async move {
            let message = json!(msg);
            let result = producer.produce("messages", message.to_string()).await;
            match result {
                Ok(_) => {}
                Err(e) => println!("{:?}", e),
            }
        });
        let fut_actor = fut.into_actor(self);
        ctx.spawn(fut_actor);
    }
}

impl SystemService for WsServer {}
impl Supervised for WsServer {}
