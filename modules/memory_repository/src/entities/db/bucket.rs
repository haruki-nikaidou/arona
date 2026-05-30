//! S3-compatible storage bucket configuration.

/// Configuration for an S3-compatible object storage bucket.
///
/// Buckets are the top-level containers for stored objects. Each bucket
/// references a credential in the vault for authentication.
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
