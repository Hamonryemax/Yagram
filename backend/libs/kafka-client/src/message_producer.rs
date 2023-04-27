use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders, ToBytes};
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use std::future::Future;
use std::time::Duration;

#[derive(Clone)]
pub struct MessageProducer {
    pub producer: FutureProducer,
    counter: u8,
}

impl MessageProducer {
    pub fn new(brokers: String) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("acks", "all")
            .set("enable.idempotence", "true")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        MessageProducer {
            producer,
            counter: 0,
        }
    }

    pub async fn produce(&mut self, topic_name: &str, message: String) -> OwnedDeliveryResult {
        self.counter = (self.counter + 1) % 5;
        let key = &format!("Key {}", self.counter);
        let record = FutureRecord::to(topic_name)
            .payload(message.to_bytes())
            .key(key);
        self.producer.send(record, Duration::from_secs(0)).await
    }
}
