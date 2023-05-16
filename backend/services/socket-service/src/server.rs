use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use kafka_messages::{KafkaMessage, StatusMessage, UserMessage};
use serde::Serialize;
use serde_json::json;

use kafka_client::MessageProducer;

pub struct WsServer {
    message_producer: MessageProducer,
}

impl Default for WsServer {
    fn default() -> Self {
        WsServer {
            message_producer: MessageProducer::new("yagram-dev-kafka:9092".to_string()),
        }
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<StatusMessage>(ctx);
        self.subscribe_system_async::<UserMessage>(ctx);
    }
}

impl Handler<StatusMessage> for WsServer {
    type Result = ();
    fn handle(&mut self, msg: StatusMessage, ctx: &mut Self::Context) -> Self::Result {
        let mut producer = self.message_producer.clone();
        let fut = Box::pin(async move {
            let kafka_message = KafkaMessage::from(msg);
            let serialized_message = json!(kafka_message);
            println!("Sending = {}", serialized_message.to_string());
            let result = producer
                .produce("messages", serialized_message.to_string(), None)
                .await;
            match result {
                Ok(_) => {}
                Err(e) => println!("{:?}", e),
            }
        });

        let fut_actor = fut.into_actor(self);
        ctx.spawn(fut_actor);
    }
}

impl Handler<UserMessage> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: UserMessage, ctx: &mut Self::Context) -> Self::Result {
        let mut producer = self.message_producer.clone();
        let fut = Box::pin(async move {
            let key = msg.key();
            let kafka_message = KafkaMessage::from(msg);
            let serialized_message = json!(kafka_message);
            let result = producer
                .produce("messages", serialized_message.to_string(), Some(key))
                .await;
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
