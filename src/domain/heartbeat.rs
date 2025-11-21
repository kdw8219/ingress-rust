use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Heartbeat {
    pub robot_id: String,
    pub is_alive: bool,
    pub ts: String,
}