use uuid::Uuid;

pub struct RollingKeyEntity {
    pub id: Uuid,
    pub encrypted_key: Vec<u8>,
    pub signature: Vec<u8>,
    pub created_at: u64,
    pub parent: Option<Uuid>,
}