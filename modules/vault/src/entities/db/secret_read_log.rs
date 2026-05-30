use time::PrimitiveDateTime;

#[derive(Debug, Clone)]
/// Logs used for audit. Each time a secret is read, a log will be created.
pub struct SecretReadLogEntity {
    pub id: i64,
    pub target: i64,
    pub timestamp: PrimitiveDateTime,
    pub version: i32,
    pub response: SecretReadResponse,
    pub wrong_password: Option<Vec<u8>>,
    pub audience: String,
    pub scope: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SecretReadResponse {
    Success,
    WrongPassword,
    SecretNotFound,
    SignatureVerificationFailed,
}
