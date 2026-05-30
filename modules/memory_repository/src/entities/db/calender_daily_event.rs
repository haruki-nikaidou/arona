//! All-day calendar events without specific times.

use time::{Date, PrimitiveDateTime};
use uuid::Uuid;

/// An all-day event that spans an entire date (no specific time).
///
/// Used for holidays, birthdays, deadlines, and other date-based
/// events that don't have a specific start/end time.
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
