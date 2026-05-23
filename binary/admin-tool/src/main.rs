//! # Isla — `admin-tool` binary
//!
//! Operator CLI for an Isla deployment.
//!
//! `admin-tool` is the "break-glass" tool for the deployment owner. It
//! deliberately **bypasses** the running `server` processes and talks
//! directly to the underlying infrastructure:
//!
//! - PostgreSQL — for inspecting and repairing persistent state.
//! - RabbitMQ  — for queue introspection and message surgery.
//! - Redis     — for cache/state inspection.
//!
//! This direct-connection design exists so that the tool stays useful even
//! when most of the cluster is down or misbehaving. It is **not** intended
//! for routine administration done through the dashboard UI — that goes
//! through the `administration` module instead.
//!
//! ## Status
//!
//! Pre-alpha. No commands are implemented yet.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

fn main() {
    println!("Hello, world!");
}
