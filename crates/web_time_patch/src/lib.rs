//! Host-neutral replacement for `web-time` in the raw Bridge wasm build.
//!
//! Ruff uses these types as a platform abstraction. The whole-file formatting
//! path does not require JavaScript clocks, so using the standard types avoids
//! introducing wasm-bindgen imports into an otherwise host-neutral module.

pub use std::time::{Duration, Instant, SystemTime, SystemTimeError, UNIX_EPOCH};
