//! Daily journal entries for long-term memory.

use time::{Date, PrimitiveDateTime};

/// A daily diary entry summarizing events and reflections.
///
/// Diaries provide a high-level record of each day, helping Isla maintain
/// continuity and recall past events without loading full conversation histories.
pub struct DiaryEntity {
    /// Unique identifier for this diary entry.
    pub id: i64,

    /// Title or headline for this day's entry.
    pub title: String,

    /// The date this entry covers.
    pub date: Date,

    /// Brief summary of the day's events.
    pub summary: String,

    /// Full diary content with reflections and details.
    pub content: String,

    /// When this entry was first written.
    pub created_at: PrimitiveDateTime,

    /// When this entry was last edited.
    pub updated_at: PrimitiveDateTime,
}