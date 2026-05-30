//! Password manager accounts for AI agent authentication.

/// A stored login credential that the AI agent can use to authenticate with external services.
///
/// Unlike generic [`SecretEntity`](super::secret::SecretEntity) entries, AI accounts are
/// structured specifically for username/password authentication flows and may include
/// TOTP configuration for two-factor authentication.
#[derive(Debug, Clone)]
pub struct AiAccountEntity {
    /// Unique identifier for this account.
    pub id: i32,

    /// Human-readable name for this account.
    pub name: String,

    /// List of website URLs or domains where this credential is valid.
    pub websites: Vec<String>,

    /// The username or email used for login.
    pub username: String,

    /// Encrypted password (ciphertext).
    pub password_encrypted: Vec<u8>,

    /// HMAC of the password for integrity verification.
    pub password_hmac: Vec<u8>,

    /// Whether this is the canonical version of the account (vs. a historical snapshot).
    pub is_master_version: bool,

    /// Unix timestamp when this account was created.
    pub created_at: u64,

    /// Soft-delete flag; if `true`, the account is no longer active.
    pub is_removed: bool,

    /// Optional TOTP provisioning URL for two-factor authentication.
    pub totp_url: Option<String>,
}
