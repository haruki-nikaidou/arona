//! # `administration` module
//!
//! Administration layer used by other Isla modules.
//!
//! This module is the routine-administration surface: it exposes a gRPC API
//! that the `dashboard` (and other modules) call to manage the deployment —
//! creating users, rotating secrets, reconfiguring plugins, viewing system
//! health, and so on.
//!
//! It is **not** the break-glass tool. When the cluster itself is in a bad
//! state, the operator uses the `admin-tool` binary, which talks directly
//! to PostgreSQL, RabbitMQ, and Redis without going through this module.
//!
//! ## Status
//!
//! Pre-alpha. No surface implemented yet.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
