//! Stored object metadata for S3-compatible storage.

/// Metadata for a file stored in object storage.
///
/// The actual file content lives in an S3-compatible bucket; this entity
/// tracks metadata and provides content-addressable lookup via the SHA-256 hash.
pub struct ObjetStorageEntity {
    /// SHA-256 hash of the file content, used as the object key.
    pub hash: [u8; 32],

    /// Original filename.
    pub file_name: String,

    /// File size in bytes.
    pub file_size: i64,

    /// Human-readable description of the file's purpose or content.
    pub description: String,

    /// MIME type or file extension (e.g., `image/png`, `pdf`).
    pub file_type: String,

    /// Unix timestamp when the object was stored.
    pub created_at: i64,

    /// ID of the [`ObjectStorageBucketEntity`](super::bucket::ObjectStorageBucketEntity) containing this object.
    pub stored_in_bucket: i32,
}
