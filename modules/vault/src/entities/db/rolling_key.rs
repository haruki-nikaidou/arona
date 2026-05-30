//! Rolling encryption keys for secret material.

use kanau::processor::Processor;
use tracing::instrument;
use uuid::Uuid;
use wakuwaku::sqlx::DatabaseProcessor;

/// A symmetric encryption key used to encrypt secrets in the vault.
///
/// Rolling keys enable key rotation without re-encrypting all secrets at once.
/// Each key is itself encrypted by a master key that is never persisted,
/// ensuring that database compromise alone does not expose secrets.
///
/// Keys form a linked list via the `before` field, allowing the vault to
/// track key lineage and decrypt older secrets during gradual rotation.
#[derive(Debug, Clone)]
pub struct RollingKeyEntity {
    /// Unique identifier for this key.
    pub id: Uuid,

    /// The symmetric key material, encrypted with the master key.
    pub encrypted_key: Vec<u8>,

    /// HMAC signature for integrity verification.
    pub signature: Vec<u8>,

    /// Unix timestamp when this key was created.
    pub created_at: i64,

    /// ID of the key this one supersedes, forming a rotation chain.
    pub before: Option<Uuid>,
}

/// Find a [`RollingKeyEntity`] by its UUID primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindRollingKeyById {
    pub id: Uuid,
}

impl Processor<FindRollingKeyById> for DatabaseProcessor {
    type Output = Option<RollingKeyEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindRollingKeyById", err, fields(id = %input.id))]
    async fn process(
        &self,
        input: FindRollingKeyById,
    ) -> Result<Option<RollingKeyEntity>, sqlx::Error> {
        sqlx::query_as!(
            RollingKeyEntity,
            r#"
            SELECT id, encrypted_key, signature, created_at, before
            FROM vault.rolling_key
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
