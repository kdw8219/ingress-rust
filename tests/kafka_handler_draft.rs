// Test drafts for `src/kafka/handler.rs`
//
// Purpose:
// - Provide a minimal, runnable plan to test `DataHandler::handling_data` behavior.
// - Show a small refactor that makes the code testable (inject a trait-backed timescale handler).
// - Provide mock-based example tests using `mockall` and a helper method to avoid constructing
//   Kafka `OwnedMessage` objects in tests.
//
// Notes:
// - These tests are drafts and marked `#[ignore]` so they don't run until you apply the
//   suggested refactor described below.
// - Two options are shown: (A) add a small helper method that accepts `topic`+`payload` for
//   test-only calls, or (B) refactor `TimescaleHandler` usage to depend on a trait that
//   can be mocked (preferred for real unit tests).

#![allow(dead_code)]

use anyhow::Result;

// Sample test using a mock. This file is a draft and tests are ignored by default.

// Suggested refactor (example snippet you can apply in `src/kafka/handler.rs`):
//
// 1) Define a trait in a new module (e.g. `crate::db::timescale::Timescale`) with
//    `async fn insert_heartbeat(&self, hb: heartbeat::Heartbeat) -> anyhow::Result<()>;`
// 2) Implement that trait for the real `TimescaleHandler`.
// 3) Change `DataHandler` to hold `Arc<dyn Timescale + Send + Sync>` instead of
//    the concrete `TimescaleHandler` type.
// 4) Optionally add a small test-only helper on `DataHandler`:
//
//    pub async fn handling_payload(&self, topic: &str, payload: &str) -> Result<()> {
//        // same parsing logic as `handling_data`, but using `topic`/`payload` directly
//    }
//
// The tests below assume such a refactor or helper exists.

#[cfg(test)]
mod tests {
    use super::*;
    use ingress_rust::domain::heartbeat;

    // Example using `mockall` to mock the timescale behaviour. Add `mockall = "0.11"`
    // and `async-trait = "0.1"` to `Cargo.toml` dev-dependencies to use this.

    // mock trait example (draft):
    //
    // mock! {
    //     pub Timescale {}
    //     #[async_trait::async_trait]
    //     trait Timescale {
    //         async fn insert_heartbeat(&self, hb: heartbeat::Heartbeat) -> anyhow::Result<()>;
    //     }
    // }

    // Example test that verifies parsing + insert_heartbeat call.
    #[tokio::test]
    #[ignore]
    async fn handling_payload_calls_insert_heartbeat() -> Result<()> {
        // Arrange: Create a mock timescale and set expectation.
        // let mut mock = MockTimescale::new();
        // mock.expect_insert_heartbeat()
        //     .times(1)
        //     .withf(|hb: &heartbeat::Heartbeat| hb.robot_id == "robot-123")
        //     .returning(|_| Box::pin(async { Ok(()) }));

        // Create the DataHandler pointing at the mock (after refactor you will be able to).
        // let dh = DataHandler::new(Arc::new(mock));

        // Sample heartbeat payload matching your domain struct.
        let sample = heartbeat::Heartbeat { robot_id: String::from("robot-123"), is_alive: true, ts: String::from("2025-11-21T00:00:00Z") };
        let payload = serde_json::to_string(&sample)?;

        // Act: call the helper that accepts topic + payload (preferred for simpler tests).
        // dh.handling_payload("", &payload).await?;

        // If you don't add a helper and cannot mock TimescaleHandler yet, you can still
        // write an integration test that runs against a test DB (not shown here).

        // Assert: expectations verified by mock when test ends.
        Ok(())
    }

    // Example test for path when payload is None. This demonstrates a small unit test
    // that doesn't need DB/mocking because nothing is inserted.
    #[tokio::test]
    #[ignore]
    async fn handling_no_payload_logs_and_returns_ok() -> Result<()> {
        // This test is illustrative: `handling_data` accepts `OwnedMessage`; to avoid
        // depending on rdkafka internals in the test, prefer adding `handling_payload`
        // helper and call it with empty payload.

        // After adding `handling_payload`, you can do:
        // let dh = DataHandler::new(Arc::new(RealTimescaleForIntegrationTests::new()));
        // dh.handling_payload("some-topic", "").await?;

        Ok(())
    }
}
