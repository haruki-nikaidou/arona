pub struct ObjetStorageEntity {
    /// Unique
    pub hash: [u8; 32],
    pub file_name: String,
    pub file_size: i64,
    pub description: String,
    pub file_type: String,
    pub created_at: i64,
    pub stored_in_bucket: i32
}
