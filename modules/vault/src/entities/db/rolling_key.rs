//! Rolling encryption keys for secret material.

use uuid::Uuid;

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
    pub created_at: u64,

    /// ID of the key this one supersedes, forming a rotation chain.
    pub before: Option<Uuid>,
}
