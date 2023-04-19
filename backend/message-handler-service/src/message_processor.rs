use actix::prelude::*;
use kafka_client::rdkafka;
use log::{info, warn};
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
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

type MessageConsumer = StreamConsumer<CustomContext>;

#[derive(Clone)]
pub struct MessageProcessor;

impl MessageProcessor {
    fn create_consumer() -> MessageConsumer {
        let context = CustomContext;
        let consumer: MessageConsumer = ClientConfig::new()
            .set("group.id", "message_handler_group_test")
            .set("bootstrap.servers", "localhost:9093")
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .expect("failed to create");

        consumer
    }
}

impl Actor for MessageProcessor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("MessageProcessor Actor started");
        let future = Box::pin(async {
            let consumer = MessageProcessor::create_consumer();
            consumer
                .subscribe(&["messages"])
                .expect("Failed to subscribe to messages topic");

            info!("Start consumer loop");
            loop {
                info!("Consumer loop iteration");
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
                        info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                            m.key(),
                            payload,
                            m.topic(),
                            m.partition(),
                            m.offset(),
                            m.timestamp()
                        );
                        if let Some(headers) = m.headers() {
                            for header in headers.iter() {
                                info!("  Header {:#?}: {:?}", header.key, header.value);
                            }
                        }
                        consumer.commit_message(&m, CommitMode::Async).unwrap();
                    }
                };
            }
        });
        let actor_future = future.into_actor(self);
        ctx.spawn(actor_future);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("MessageProcessor Actor stopped");
    }
}
