//! Memorable events and interactions with contacts.

use time::PrimitiveDateTime;
use uuid::Uuid;

/// A notable event or interaction involving a contact.
///
/// Stories capture significant moments in Isla's relationship with a person,
/// such as first meetings, relationship changes, or memorable conversations.
pub struct ContactStoryEntity {
    /// Unique identifier for this story.
    pub id: i64,

    /// ID of the [`ContactIdentityEntity`](super::contact_identity::ContactIdentityEntity) this story is about.
    pub identity: Uuid,

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
