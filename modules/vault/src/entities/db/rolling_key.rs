use uuid::Uuid;

#[derive(Debug, Clone)]
/// Keys used for symmetric encrypting. All keys are encrypted by the master key which is not stored
/// anywhere.
pub struct RollingKeyEntity {
    pub id: Uuid,
    pub encrypted_key: Vec<u8>,
    pub signature: Vec<u8>,
    pub created_at: u64,
    pub before: Option<Uuid>,
}
