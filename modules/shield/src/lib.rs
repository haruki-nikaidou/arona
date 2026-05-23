//! # `shield` module
//!
//! WAF-like protection layer for Isla.
//!
//! `shield` sits in front of user-facing surfaces and protects them from
//! abuse. Planned responsibilities include:
//!
//! - Anti-XSS filtering / output hardening.
//! - CAPTCHA challenges and similar bot-deterrents.
//! - Rate limiting and other tools to prevent unauthorized access.
//!
//! The exact integration boundary — proxy, middleware, or in-process gRPC
//! interceptors — is not yet decided.
//!
//! ## Status
//!
//! Pre-alpha. Currently contains only a placeholder `add` function from
//! `cargo new`.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
