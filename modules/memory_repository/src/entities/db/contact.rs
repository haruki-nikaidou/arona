//! Platform-specific contact records.

use kanau::processor::Processor;
use time::PrimitiveDateTime;
use tracing::instrument;
use uuid::Uuid;
use wakuwaku::sqlx::DatabaseProcessor;

/// A contact as they appear on a specific platform.
///
/// The same person may have multiple `ContactEntity` records (one per platform),
/// but they share a single [`ContactIdentityEntity`](super::contact_identity::ContactIdentityEntity)
/// that represents the real-world identity Isla has recognized.
#[derive(Debug, Clone)]
pub struct ContactEntity {
    /// Unique identifier for this contact record.
    pub id: i64,

    /// The name shown for this contact on the platform.
    pub display_name: String,

    /// Platform-specific user identifier.
    pub user_id: String,

    /// Platform name (e.g., `discord`, `telegram`, `slack`).
    pub platform: String,

    /// Foreign key to the
    /// [`ContactIdentityEntity`](super::contact_identity::ContactIdentityEntity)
    /// this contact is linked to.
    pub identity: Uuid,

    /// When this contact was first seen.
    pub created_at: PrimitiveDateTime,

    /// When this contact record was last updated.
    pub updated_at: PrimitiveDateTime,
}

/// Find a [`ContactEntity`] by its bigserial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindContactById {
    pub id: i64,
}

impl Processor<FindContactById> for DatabaseProcessor {
    type Output = Option<ContactEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindContactById", err, fields(id = input.id))]
    async fn process(&self, input: FindContactById) -> Result<Option<ContactEntity>, sqlx::Error> {
        sqlx::query_as!(
            ContactEntity,
            r#"
            SELECT id, display_name, user_id, platform, identity, created_at, updated_at
            FROM memory.contact
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
