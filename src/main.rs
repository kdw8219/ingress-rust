use rdkafka::{Message, consumer::StreamConsumer};
use futures::StreamExt;

mod config;
mod kafka;

//요구사항 정리
// async로 동작필요. 따라서 main은 async로 동작할 수 있는 eventloop 같은 걸 계속 돌려야 함
// async loop 에서는 Kafka에서 데이터를 가져오고, handler에 각 data type 별로 처리 방식을 나누어야 함
// Kafka에서 들어오는 데이터는 JSON이므로 이를 파싱하고 처리할 방법 필요

#[tokio::main]
async fn main() {
    let settings = config::configs::load_settings();
    let kafka_config: config::configs::KafkaConfig = settings.kafka;
    let consumer: StreamConsumer = kafka::consumer::create_consumer(kafka_config)
                                    .expect("failed to create consumer");
    
    let mut stream = consumer.stream();
    
    while let Some(result) = stream.next().await{
        match result {
            Ok(msg) => {
                //msg 가지고 tokio spawn해서 별도 task로 처리하게끔 함(thread 동작. 싱글 스레드 아님. 코루틴 개념(즉, OS 개념 아님))
                //이걸 handler로 던져서 tokio spawn 하게 하면 될듯
            }
            Err(e) => {
                eprint!("Kafka handling Error: {}", e);
            }
        }
    }

    //serde_json::Deserializer
}
