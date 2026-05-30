//! Time-specific calendar events.

use kanau::processor::Processor;
use time::{Date, OffsetDateTime, PrimitiveDateTime};
use tracing::instrument;
use uuid::Uuid;
use wakuwaku::sqlx::DatabaseProcessor;

/// A calendar event with a specific start time.
///
/// Used for meetings, appointments, reminders, and other events
/// that occur at a particular moment.
#[derive(Debug, Clone)]
pub struct CalenderEventEntity {
    /// Unique identifier for this event.
    pub id: i64,

    /// ID of the parent [`CalenderEntity`](super::calender::CalenderEntity).
    pub calendar_id: Uuid,

    /// Event title.
    pub title: String,

    /// Detailed description of the event.
    pub description: String,

    /// Start time of the event (with timezone).
    pub time: OffsetDateTime,

    /// Recurrence pattern for this event.
    pub repeat: CalenderEventRepeat,

    /// End date for recurring events (inclusive).
    pub repeat_until: Option<Date>,

    /// When this event was created.
    pub created_at: PrimitiveDateTime,

    /// When this event was last modified.
    pub updated_at: PrimitiveDateTime,
}

/// Recurrence pattern for time-specific events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "memory.calender_event_repeat")]
pub enum CalenderEventRepeat {
    /// Single occurrence, no repetition.
    NoRepeat,
    /// Repeats at the same time every day.
    EveryDay,
    /// Repeats on the same day and time each month.
    EveryMonth,
    /// Repeats at the same time every weekday.
    EveryWeekday,
}

/// Find a [`CalenderEventEntity`] by its bigserial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindCalenderEventById {
    pub id: i64,
}

impl Processor<FindCalenderEventById> for DatabaseProcessor {
    type Output = Option<CalenderEventEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindCalenderEventById", err, fields(id = input.id))]
    async fn process(
        &self,
        input: FindCalenderEventById,
    ) -> Result<Option<CalenderEventEntity>, sqlx::Error> {
        sqlx::query_as!(
            CalenderEventEntity,
            r#"
            SELECT
                id,
                calendar_id,
                title,
                description,
                time,
                repeat AS "repeat: CalenderEventRepeat",
                repeat_until,
                created_at,
                updated_at
            FROM memory.calender_event
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
