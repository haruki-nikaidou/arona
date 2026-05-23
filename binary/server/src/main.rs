//! # Isla — `server` binary
//!
//! The long-running service binary for an Isla deployment.
//!
//! A single Isla cluster is expected to run this binary multiple times, each
//! invocation selecting a different *worker mode* (a group of core modules to
//! host in that process). Modes are intentionally a small fixed set rather
//! than one-mode-per-module: each mode bundles related modules so that a
//! typical deployment runs just a handful of processes while still keeping
//! clear functional separation.
//!
//! Concrete worker mode names are not yet finalized; see [`workers`].
//!
//! ## Talking to the cluster
//!
//! - Intra-cluster module-to-module: gRPC.
//! - Plugins: JSON over RabbitMQ (AMQP), via `plugin_registrar`.
//! - Persistent state: PostgreSQL.
//! - Caches / ephemeral state: Redis.
//!
//! Operator-side recovery and break-glass actions do **not** go through this
//! binary; for that, see the `admin-tool` binary, which talks directly to
//! Redis, RabbitMQ, and PostgreSQL.
//!
//! ## Status
//!
//! Pre-alpha. Almost nothing is wired up yet.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub mod workers;

fn main() {
    println!("Hello, world!");
}
