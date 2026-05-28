use time::PrimitiveDateTime;

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

pub enum SecretReadResponse {
    Success,
    WrongPassword,
    SecretNotFound,
    SignatureVerificationFailed,
}
