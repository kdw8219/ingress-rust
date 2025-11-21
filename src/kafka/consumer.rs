use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
//use rdkafka::message::BorrowedMessage;
use crate::config::configs;
use futures::StreamExt;
use std::sync::Arc;
use tokio;

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

pub async fn run_consumer(kafka_config: configs::KafkaConfig, ts_config: configs::TimeSeriesConfig) -> Result< (), anyhow::Error> {
    let consumer = create_consumer(kafka_config)?;
    let handler = Arc::new(crate::kafka::handler::DataHandler::create_handler(ts_config).await?);
    let mut stream = consumer.stream();
    
    while let Some(result) = stream.next().await{
        match result {
            Ok(msg) => {
                //msg 가지고 tokio spawn해서 별도 task로 처리하게끔 함(thread 동작. 싱글 스레드 아님. 코루틴 개념(즉, OS 개념 아님))
                //이걸 handler로 던져서 tokio spawn 하게 하면 될듯
                let owned = msg.detach();
                let h = handler.clone();

                tokio::spawn(async move {
                    h.handling_data(owned).await;
                });
            }
            Err(e) => {
                eprint!("Kafka handling Error: {}", e);
                
            }
        }
    }

    Ok(())
}