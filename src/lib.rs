// Library entry so integration tests (in `tests/`) can access crate items.
// Keep this minimal: re-export the same modules used by `src/main.rs`.

pub mod config;
pub mod db;
pub mod domain;
pub mod kafka;

// Re-export commonly-used items (optional). Add more `pub use` entries as needed.
pub use domain::heartbeat;
