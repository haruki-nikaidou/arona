//! Time-specific calendar events.

use time::{Date, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

/// A calendar event with a specific start time.
///
/// Used for meetings, appointments, reminders, and other events
/// that occur at a particular moment.
pub struct CalenderEventEntity {
    /// Unique identifier for this event.
    pub id: i64,

    /// ID of the parent [`CalendarEntity`](super::calender::CalendarEntity).
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
