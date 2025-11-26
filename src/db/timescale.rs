use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use anyhow::Result;

use crate::domain::heartbeat;
use crate::config::configs::TimeSeriesConfig;

pub struct TimescaleHandler {
    pool: Pool<Postgres>
}

impl TimescaleHandler {
    pub async  fn create_handler(ts_config:TimeSeriesConfig) -> Result<TimescaleHandler, anyhow::Error> {

        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            ts_config.user,
            ts_config.password,
            ts_config.host,
            ts_config.port,
            ts_config.dbname,
        );

        let pool = PgPoolOptions::new()
                        .max_connections(10)
                        .min_connections(2)
                        .acquire_timeout(std::time::Duration::from_secs(5))
                        .idle_timeout(std::time::Duration::from_secs(600))
                        .max_lifetime(std::time::Duration::from_secs(18000))
                        .connect(&url)
                        .await?;
        Ok(
            Self {
                pool:pool,
            }
        )
    }

    pub async fn insert_heartbeat(&self, hb:heartbeat::Heartbeat) -> Result<()> {
        println!("start to insert heartbeat");
        //heart beat은 batch insert 하면 안된다.
        sqlx::query(
            r#"
            INSERT INTO robot_heartbeat (robot_id, is_alive, timestamp)
            VALUES ($1, $2, $3)
            "#
        )
        .bind(&hb.robot_id)
        .bind(hb.is_alive)
        .bind(hb.timestamp)
        .execute(&self.pool)
        .await?;
        println!("End to insert heartbeat");
        Ok(())
    }

}