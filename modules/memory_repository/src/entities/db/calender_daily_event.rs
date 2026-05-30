//! All-day calendar events without specific times.

use kanau::processor::Processor;
use time::{Date, PrimitiveDateTime};
use tracing::instrument;
use uuid::Uuid;
use wakuwaku::sqlx::DatabaseProcessor;

/// An all-day event that spans an entire date (no specific time).
///
/// Used for holidays, birthdays, deadlines, and other date-based
/// events that don't have a specific start/end time.
#[derive(Debug, Clone)]
pub struct CalenderDailyEventEntity {
    /// Unique identifier for this event.
    pub id: i64,

    /// ID of the parent [`CalenderEntity`](super::calender::CalenderEntity).
    pub calendar_id: Uuid,

    /// Event title.
    pub title: String,

    /// Detailed description of the event.
    pub description: String,

    /// The date of this event (or first occurrence if repeating).
    pub date: Date,

    /// Recurrence pattern for this event.
    pub repeat: DailyEventRepeat,

    /// End date for recurring events (inclusive).
    pub repeat_until: Option<Date>,

    /// When this event was created.
    pub created: PrimitiveDateTime,

    /// When this event was last modified.
    pub updated: PrimitiveDateTime,
}

/// Recurrence pattern for all-day events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "memory.daily_event_repeat")]
pub enum DailyEventRepeat {
    /// Single occurrence, no repetition.
    NoRepeat,
    /// Repeats on the same day each month.
    EveryMonth,
    /// Repeats on the same date each year (e.g., birthdays).
    EveryYear,
    /// Repeats every weekday (Monday through Friday).
    EveryWeekday,
}

/// Find a [`CalenderDailyEventEntity`] by its bigserial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindCalenderDailyEventById {
    pub id: i64,
}

impl Processor<FindCalenderDailyEventById> for DatabaseProcessor {
    type Output = Option<CalenderDailyEventEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindCalenderDailyEventById", err, fields(id = input.id))]
    async fn process(
        &self,
        input: FindCalenderDailyEventById,
    ) -> Result<Option<CalenderDailyEventEntity>, sqlx::Error> {
        sqlx::query_as!(
            CalenderDailyEventEntity,
            r#"
            SELECT
                id,
                calendar_id,
                title,
                description,
                date,
                repeat AS "repeat: DailyEventRepeat",
                repeat_until,
                created,
                updated
            FROM memory.calender_daily_event
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
