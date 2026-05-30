use uuid::Uuid;

#[derive(Debug, Clone)]
/// API tokens and other kinds of secrets. They are encrypted by [RollingKeyEntity](super::rolling_key::RollingKeyEntity).
pub struct SecretEntity {
    pub id: i64,
    pub platform: String,
    pub name: String,
    pub allowed_scopes: Vec<String>,
    pub content: Vec<u8>,
    pub signature: Vec<u8>,
    pub key: Uuid,
    pub created_at: u64,
    pub updated_at: u64,
    pub version: i32,
}
