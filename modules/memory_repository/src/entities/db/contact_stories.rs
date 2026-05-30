//! Memorable events and interactions with contacts.

use kanau::processor::Processor;
use time::PrimitiveDateTime;
use tracing::instrument;
use uuid::Uuid;
use wakuwaku::sqlx::DatabaseProcessor;

/// A notable event or interaction involving a contact.
///
/// Stories capture significant moments in Isla's relationship with a person,
/// such as first meetings, relationship changes, or memorable conversations.
#[derive(Debug, Clone)]
pub struct ContactStoryEntity {
    /// Unique identifier for this story.
    pub id: i64,

    /// ID of the [`ContactIdentityEntity`](super::contact_identity::ContactIdentityEntity) this story is about.
    pub identity: Uuid,

    /// Classification of what kind of event this story represents.
    pub story_type: StoryType,

    /// Short title for this story.
    pub story_name: String,

    /// Brief summary of what happened.
    pub story_summary: String,

    /// Full narrative of the event.
    pub story_text: String,

    /// When this event occurred.
    pub happened_at: PrimitiveDateTime,

    /// Optional link to the conversation where this story originated.
    pub related_conversation: Option<i64>,
}

/// Classification of what kind of event a story represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "memory.story_type")]
pub enum StoryType {
    /// The relationship became closer or more trusting.
    RelationshipUpgrade,
    /// The relationship became more distant or strained.
    RelationshipDowngrade,
    /// The first interaction with this person.
    FirstMeeting,
    /// Isla's understanding of this person changed significantly.
    ImpressionChanged,
    /// Any other notable event.
    Other,
}

/// Find a [`ContactStoryEntity`] by its bigserial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindContactStoryById {
    pub id: i64,
}

impl Processor<FindContactStoryById> for DatabaseProcessor {
    type Output = Option<ContactStoryEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindContactStoryById", err, fields(id = input.id))]
    async fn process(
        &self,
        input: FindContactStoryById,
    ) -> Result<Option<ContactStoryEntity>, sqlx::Error> {
        sqlx::query_as!(
            ContactStoryEntity,
            r#"
            SELECT
                id,
                identity,
                story_type AS "story_type: StoryType",
                story_name,
                story_summary,
                story_text,
                happened_at,
                related_conversation
            FROM memory.contact_story
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
