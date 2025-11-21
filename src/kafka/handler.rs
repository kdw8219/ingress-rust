use rdkafka::{Message, message::OwnedMessage};

use crate::db::timescale;
use crate::domain::heartbeat;
use anyhow::Result;
use crate::config::configs::TimeSeriesConfig;

pub struct DataHandler {
    timescale_handler: timescale::TimescaleHandler
}

impl DataHandler {
    pub async fn create_handler(ts_config: TimeSeriesConfig) -> Result<Self, anyhow::Error> {
        let time_scaler = timescale::TimescaleHandler::create_handler(ts_config).await?;
        
        Ok(
            Self {
                timescale_handler: time_scaler,
            }
        )
    }

    pub async fn handling_data(&self, msg:OwnedMessage) -> Result<()> {

        if let Some(payload) = msg.payload(){
            let topic = msg.topic();
            let data = msg.payload().map(|s| String::from_utf8_lossy(s)
                                                .to_string())
                                                .unwrap_or_default();

            println!("topic: {}", topic);

            //topic 기준으로 데이터 쪼개기
            if(topic == "") {
                let hb: heartbeat::Heartbeat = serde_json::from_str(&data)?;
                self.timescale_handler.insert_heartbeat(hb).await;
            }
            else {
                println!("Unsupported topic: {}", topic);            
            }
        }
        else {
            println!("No payload!")
        }

        Ok(())
    }
}

