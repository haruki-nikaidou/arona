//! Generic secret storage for API tokens and credentials.

use kanau::processor::Processor;
use tracing::instrument;
use uuid::Uuid;
use wakuwaku::sqlx::DatabaseProcessor;

/// A stored secret such as an API token, OAuth refresh token, or service credential.
///
/// Secrets are encrypted using a [`RollingKeyEntity`](super::rolling_key::RollingKeyEntity)
/// and access is controlled by scope-based permissions.
#[derive(Debug, Clone)]
pub struct SecretEntity {
    /// Unique identifier for this secret.
    pub id: i64,

    /// The external platform or service this secret is associated with (e.g., `github`, `slack`).
    pub platform: String,

    /// Human-readable name for this secret.
    pub name: String,

    /// List of scope patterns that are permitted to access this secret.
    pub allowed_scopes: Vec<String>,

    /// Encrypted secret content (ciphertext).
    pub content: Vec<u8>,

    /// HMAC signature for integrity verification.
    pub signature: Vec<u8>,

    /// ID of the [`RollingKeyEntity`](super::rolling_key::RollingKeyEntity) used to encrypt this secret.
    pub key: Uuid,

    /// Unix timestamp when this secret was created.
    pub created_at: i64,

    /// Unix timestamp when this secret was last modified.
    pub updated_at: i64,

    /// Optimistic concurrency version number.
    pub version: i32,
}

/// Find a [`SecretEntity`] by its bigserial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindSecretById {
    pub id: i64,
}

impl Processor<FindSecretById> for DatabaseProcessor {
    type Output = Option<SecretEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindSecretById", err, fields(id = input.id))]
    async fn process(&self, input: FindSecretById) -> Result<Option<SecretEntity>, sqlx::Error> {
        sqlx::query_as!(
            SecretEntity,
            r#"
            SELECT
                id,
                platform,
                name,
                allowed_scopes,
                content,
                signature,
                key,
                created_at,
                updated_at,
                version
            FROM vault.secret
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
