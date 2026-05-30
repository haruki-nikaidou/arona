//! Stored object metadata for S3-compatible storage.

use kanau::processor::Processor;
use tracing::instrument;
use wakuwaku::sqlx::DatabaseProcessor;

/// Metadata for a file stored in object storage.
///
/// The actual file content lives in an S3-compatible bucket; this entity
/// tracks metadata and provides content-addressable lookup via the SHA-256 hash.
#[derive(Debug, Clone)]
pub struct ObjetStorageEntity {
    /// SHA-256 hash of the file content, used as the object key.
    /// The DB enforces `octet_length(hash) = 32` via a CHECK constraint.
    pub hash: Vec<u8>,

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

/// Find an [`ObjetStorageEntity`] by its SHA-256 content hash (the table's primary key).
#[derive(Debug, Clone)]
pub struct FindObjectStorageByHash {
    pub hash: Vec<u8>,
}

impl Processor<FindObjectStorageByHash> for DatabaseProcessor {
    type Output = Option<ObjetStorageEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindObjectStorageByHash", err)]
    async fn process(
        &self,
        input: FindObjectStorageByHash,
    ) -> Result<Option<ObjetStorageEntity>, sqlx::Error> {
        sqlx::query_as!(
            ObjetStorageEntity,
            r#"
            SELECT hash, file_name, file_size, description, file_type, created_at, stored_in_bucket
            FROM memory.object_storage
            WHERE hash = $1
            LIMIT 1
            "#,
            &input.hash,
        )
        .fetch_optional(self.db())
        .await
    }
}
