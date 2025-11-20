use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use crate::config::configs;

pub fn create_consumer(kafka_config: configs::KafkaConfig) -> Result< StreamConsumer, anyhow::Error> {
    let consumer: StreamConsumer = ClientConfig::new()
    .set("bootstrap.servers", &kafka_config.bootstrap_servers)
    .set("group.id", &kafka_config.group_id)
    .set("auto.offset.reset", &kafka_config.auto_offset_reset)
    .create()?;

    let topic_refs: Vec<&str> = kafka_config.topics.iter().map(|x| x.as_str()).collect();

    consumer.subscribe(&topic_refs)?;

    Ok(consumer)
}