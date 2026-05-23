//! # `wakuwaku`
//!
//! Internal infrastructure crate for the Isla workspace.
//!
//! `wakuwaku` is the place where cross-cutting plumbing lives so that the
//! core modules don't each reinvent it. It is **not** a public-facing crate
//! and makes no stability promises outside of this workspace.
//!
//! Provided pieces:
//!
//! - [`pool`]         — a generic async connection pool used by the
//!                      AMQP / SQL / Redis adapters below.
//! - [`amqp`]         — RabbitMQ helpers built on `amqprs`
//!                      *(enabled by the `amqprs` feature)*.
//! - [`sqlx`]         — PostgreSQL helpers built on `sqlx`
//!                      *(enabled by the `sqlx` feature)*.
//! - [`redis`]        — Redis helpers
//!                      *(enabled by the `redis` feature)*.
//! - [`interval_job`] — small interval-driven background job runner.
//! - [`error`]        — shared error type, re-exported as
//!                      [`Error`].
//!
//! ## Features
//!
//! The default feature set enables `amqprs`, `sqlx`, `redis`, `uuid`, and
//! `tracing`. `tracing-otel` opts into OpenTelemetry-flavored tracing.
//!
//! ## Status
//!
//! Pre-alpha, like the rest of the workspace.

#[cfg(feature = "amqprs")]
pub mod amqp;

pub mod error;
pub mod interval_job;
pub mod pool;

#[cfg(feature = "sqlx")]
pub mod sqlx;

#[cfg(feature = "redis")]
pub mod redis;

pub use error::Error;
