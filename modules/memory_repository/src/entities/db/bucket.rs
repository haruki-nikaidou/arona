//! S3-compatible storage bucket configuration.

use kanau::processor::Processor;
use tracing::instrument;
use wakuwaku::sqlx::DatabaseProcessor;

/// Configuration for an S3-compatible object storage bucket.
///
/// Buckets are the top-level containers for stored objects. Each bucket
/// references a credential in the vault for authentication.
#[derive(Debug, Clone)]
pub struct ObjectStorageBucketEntity {
    /// Unique identifier for this bucket configuration.
    pub id: i32,

    /// Human-readable display name for this bucket.
    pub readable_name: String,

    /// The actual bucket name in the storage provider.
    pub bucket_name: String,

    /// Storage region (e.g., `us-east-1`, `eu-west-1`).
    pub region: String,

    /// S3-compatible API endpoint URL.
    pub api_endpoint: String,

    /// Reference to the vault credential used for bucket access.
    pub credential: i64,
}

/// Find an [`ObjectStorageBucketEntity`] by its serial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindObjectStorageBucketById {
    pub id: i32,
}

impl Processor<FindObjectStorageBucketById> for DatabaseProcessor {
    type Output = Option<ObjectStorageBucketEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindObjectStorageBucketById", err, fields(id = input.id))]
    async fn process(
        &self,
        input: FindObjectStorageBucketById,
    ) -> Result<Option<ObjectStorageBucketEntity>, sqlx::Error> {
        sqlx::query_as!(
            ObjectStorageBucketEntity,
            r#"
            SELECT id, readable_name, bucket_name, region, api_endpoint, credential
            FROM memory.object_storage_bucket
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
