//! # `gmail` plugin
//!
//! Reference Isla plugin for Gmail.
//!
//! - **Namespace:** `office.gmail`.
//! - **Transport:** JSON over RabbitMQ (AMQP), via `plugin_registrar`.
//!
//! This crate exists primarily as a working example of the plugin contract
//! described in the top-level `README.md`. It is shipped in-repo for
//! convenience but is not part of the Isla core — third-party plugins are
//! written and deployed exactly the same way.
//!
//! ## Status
//!
//! Pre-alpha. Currently just a stub `main`.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

fn main() {
    println!("Hello, world!");
}
