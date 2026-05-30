#[derive(Debug, Clone)]
/// Password manager account entity for accounts used by AI.
pub struct AiAccountEntity {
    pub id: i32,
    pub name: String,
    pub websites: Vec<String>,
    pub username: String,
    pub password_encrypted: Vec<u8>,
    pub password_hmac: Vec<u8>,
    pub is_master_version: bool,
    pub created_at: u64,
    pub is_removed: bool,
    pub totp_url: Option<String>,
}
