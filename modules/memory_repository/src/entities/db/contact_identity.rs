//! Cross-platform identity and relationship tracking.

use time::PrimitiveDateTime;
use uuid::Uuid;

/// A unified identity representing a real person across platforms.
///
/// Multiple [`ContactEntity`](super::contact::ContactEntity) records from
/// different platforms can link to the same identity, allowing Isla to
/// maintain a consistent understanding of relationships.
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
