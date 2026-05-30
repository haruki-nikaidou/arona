//! Audit logging for secret access.

use time::PrimitiveDateTime;

/// An audit log entry recording an attempt to read a secret.
///
/// Every secret access — successful or not — is logged for security auditing.
/// Failed attempts include additional context (e.g., the incorrect password hash)
/// to aid forensic analysis.
#[derive(Debug, Clone)]
pub struct SecretReadLogEntity {
    /// Unique identifier for this log entry.
    pub id: i64,

    /// ID of the [`SecretEntity`](super::secret::SecretEntity) that was accessed.
    pub target: i64,

    /// When the access attempt occurred.
    pub timestamp: PrimitiveDateTime,

    /// Version of the secret at the time of access.
    pub version: i32,

    /// Outcome of the access attempt.
    pub response: SecretReadResponse,

    /// If authentication failed, the (hashed) incorrect password provided.
    pub wrong_password: Option<Vec<u8>>,

    /// Identifier of the service or user that requested the secret.
    pub audience: String,

    /// The scope under which access was requested.
    pub scope: String,
}

/// Outcome of a secret read operation.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SecretReadResponse {
    /// The secret was successfully decrypted and returned.
    Success,

    /// Authentication failed due to an incorrect password.
    WrongPassword,

    /// No secret exists with the requested identifier.
    SecretNotFound,

    /// The secret's integrity signature did not match (possible tampering).
    SignatureVerificationFailed,
}
