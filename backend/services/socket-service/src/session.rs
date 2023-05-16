use crate::server::WsServer;
use actix::{fut, prelude::*};
use actix_broker::BrokerIssue;
use actix_web_actors::ws;
use kafka_client::rdkafka;
use kafka_messages::{ClientUserMessageData, KafkaMessage, StatusMessage, UserMessage};
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        println!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        println!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        println!("Committing offsets: {:?}", result);
    }
}

type MessageConsumer = StreamConsumer<CustomContext>;

pub struct WsSession {
    user_id: String,
}

impl WsSession {
    pub fn new(user_id: String) -> Self {
        Self { user_id }
    }
    fn process_text_message(&self, message: &str) {
        if let Ok(message_info) = serde_json::from_str::<ClientUserMessageData>(message) {
            self.issue_system_async(UserMessage::from_client(self.user_id.clone(), message_info));
        }
    }

    async fn read_messages() {
        let context = CustomContext;
        let consumer: MessageConsumer = ClientConfig::new()
            .set("group.id", "message_reader_test_1")
            .set("bootstrap.servers", "yagram-dev-kafka:9092")
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .expect("failed to create");

        consumer
            .subscribe(&["messages"])
            .expect("Failed to subscribe to messages topic");

        println!("Start consumer loop");
        loop {
            println!("Consumer loop iteration");
            match consumer.recv().await {
                Err(e) => warn!("Kafka error: {}", e),
                Ok(m) => {
                    let payload = match m.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            warn!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                    };

                    let payload = serde_json::from_str::<KafkaMessage>(payload);

                    println!("key: '{:?}', payload: '{:?}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                            m.key(),
                            payload,
                            m.topic(),
                            m.partition(),
                            m.offset(),
                            m.timestamp()
                        );
                    if let Some(headers) = m.headers() {
                        for header in headers.iter() {
                            println!("  Header {:#?}: {:?}", header.key, header.value);
                        }
                    }
                    consumer.commit_message(&m, CommitMode::Async).unwrap();
                }
            };
        }
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        self.issue_system_async(StatusMessage::connect(self.user_id.clone()));
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.issue_system_async(StatusMessage::disconnect(self.user_id.clone()));
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
