//! # `telegram_bot` — Telegram user interface
//!
//! First-party Telegram adapter for Isla.
//!
//! This binary is a thin shim between the Telegram Bot API and the
//! `interface` module: it forwards inbound user messages into the cluster
//! over gRPC and renders outbound messages from the cluster back to
//! Telegram chats.
//!
//! It is *not* a plugin — it provides no tools and no skills. "Send a
//! message" tooling lives inside the `interface` module itself, so this
//! adapter only has to translate between transport formats. Other chat
//! platforms (Discord, Slack, …) are added as siblings under
//! `user_interface/` following the same pattern.
//!
//! ## Status
//!
//! Pre-alpha. Currently just a stub `main`.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

fn main() {
    println!("Hello, world!");
}
