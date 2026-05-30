//! Daily journal entries for long-term memory.

use kanau::processor::Processor;
use time::{Date, PrimitiveDateTime};
use tracing::instrument;
use wakuwaku::sqlx::DatabaseProcessor;

/// A daily diary entry summarizing events and reflections.
///
/// Diaries provide a high-level record of each day, helping Isla maintain
/// continuity and recall past events without loading full conversation histories.
#[derive(Debug, Clone)]
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

/// Find a [`DiaryEntity`] by its bigserial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindDiaryById {
    pub id: i64,
}

impl Processor<FindDiaryById> for DatabaseProcessor {
    type Output = Option<DiaryEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindDiaryById", err, fields(id = input.id))]
    async fn process(&self, input: FindDiaryById) -> Result<Option<DiaryEntity>, sqlx::Error> {
        sqlx::query_as!(
            DiaryEntity,
            r#"
            SELECT id, title, date, summary, content, created_at, updated_at
            FROM memory.diary
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}