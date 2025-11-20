use serde::Deserialize;
use config::{Config, File as ConfigFile};

#[derive(Deserialize, Debug)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub group_id: String,
    pub topics: Vec<String>,
    pub auto_offset_reset: String,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub kafka: KafkaConfig,
}

pub fn load_settings() -> Settings {
    Config::builder().add_source(ConfigFile::with_name("config/default"))
    .build()
    .unwrap()
    .try_deserialize()
    .unwrap()
}
