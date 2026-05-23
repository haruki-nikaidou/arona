//! # `interface` module
//!
//! Unified user-facing channel abstraction.
//!
//! Isla can be reached through several channels — the built-in `webui`, plus
//! chat bots for Telegram, Discord, Slack, and so on. This module exposes a
//! single gRPC API that:
//!
//! - lets the rest of the cluster send messages to a user without caring
//!   which platform that user is on, and
//! - lets each platform adapter (such as `user_interface/telegram_bot`)
//!   deliver inbound user messages into the cluster through one common
//!   protocol.
//!
//! The "send a message" tool exposed to the model lives here as well, rather
//! than in a plugin, because it is fundamental to every channel and is not
//! optional.
//!
//! ## Status
//!
//! Pre-alpha. Nothing implemented yet.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
