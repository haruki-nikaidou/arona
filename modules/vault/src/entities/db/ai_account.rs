//! Password manager accounts for AI agent authentication.

use kanau::processor::Processor;
use tracing::instrument;
use wakuwaku::sqlx::DatabaseProcessor;

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
    pub created_at: i64,

    /// Soft-delete flag; if `true`, the account is no longer active.
    pub is_removed: bool,

    /// Optional TOTP provisioning URL for two-factor authentication.
    pub totp_url: Option<String>,
}

/// Find an [`AiAccountEntity`] by its serial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindAiAccountById {
    pub id: i32,
}

impl Processor<FindAiAccountById> for DatabaseProcessor {
    type Output = Option<AiAccountEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindAiAccountById", err, fields(id = input.id))]
    async fn process(
        &self,
        input: FindAiAccountById,
    ) -> Result<Option<AiAccountEntity>, sqlx::Error> {
        sqlx::query_as!(
            AiAccountEntity,
            r#"
            SELECT
                id,
                name,
                websites,
                username,
                password_encrypted,
                password_hmac,
                is_master_version,
                created_at,
                is_removed,
                totp_url
            FROM vault.ai_account
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
