//! Cross-platform identity and relationship tracking.

use kanau::processor::Processor;
use time::PrimitiveDateTime;
use tracing::instrument;
use uuid::Uuid;
use wakuwaku::sqlx::DatabaseProcessor;

/// A unified identity representing a real person across platforms.
///
/// Multiple [`ContactEntity`](super::contact::ContactEntity) records from
/// different platforms can link to the same identity, allowing Isla to
/// maintain a consistent understanding of relationships.
#[derive(Debug, Clone)]
pub struct ContactIdentityEntity {
    /// Unique identifier for this identity.
    pub id: Uuid,

    /// The name Isla uses to refer to this person internally.
    pub identify_name: String,

    /// Notes about this person (interests, context, etc.).
    pub description: String,

    /// Current relationship status with this person.
    pub relationship: Relationship,

    /// When Isla first interacted with this person.
    pub first_meet_at: PrimitiveDateTime,

    /// When the relationship status was last changed.
    pub relationship_updated_at: PrimitiveDateTime,
}

/// The type of relationship Isla has with a contact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "memory.relationship")]
pub enum Relationship {
    /// Unknown person with no established relationship.
    Stranger,
    /// The primary user who owns/controls this Isla instance.
    Master,
    /// Someone Isla has interacted with but doesn't know well.
    Acquaintance,
    /// A friend or close contact.
    Dude,
    /// A contact that should be deprioritized or filtered.
    Ignored,
}

/// Find a [`ContactIdentityEntity`] by its UUID primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindContactIdentityById {
    pub id: Uuid,
}

impl Processor<FindContactIdentityById> for DatabaseProcessor {
    type Output = Option<ContactIdentityEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindContactIdentityById", err, fields(id = %input.id))]
    async fn process(
        &self,
        input: FindContactIdentityById,
    ) -> Result<Option<ContactIdentityEntity>, sqlx::Error> {
        sqlx::query_as!(
            ContactIdentityEntity,
            r#"
            SELECT
                id,
                identify_name,
                description,
                relationship AS "relationship: Relationship",
                first_meet_at,
                relationship_updated_at
            FROM memory.contact_identity
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
