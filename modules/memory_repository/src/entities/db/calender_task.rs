//! Task management within calendars.

use time::PrimitiveDateTime;
use uuid::Uuid;

/// A task with a deadline, tracked within a calendar.
///
/// Tasks differ from events in that they have a duration window (start to deadline)
/// and progress tracking, rather than occurring at a specific moment.
pub struct CalenderTaskEntity {
    /// Unique identifier for this task.
    pub id: i64,

    /// ID of the parent [`CalenderEntity`](super::calender::CalenderEntity).
    pub calendar_id: Uuid,

    /// Task title.
    pub title: String,

    /// Detailed description of what needs to be done.
    pub description: String,

    /// When work on this task can begin.
    pub start_at: PrimitiveDateTime,

    /// When this task must be completed.
    pub deadline: PrimitiveDateTime,

    /// When the task status was last changed.
    pub status_update_at: PrimitiveDateTime,
}

/// Progress state of a task.
pub enum CalenderTaskStatus {
    /// Task has not been started.
    Pending,
    /// Task is currently being worked on.
    Doing,
    /// Task has been completed.
    Finished,
}

/// A dependency relationship between two tasks.
///
/// Represents that one task must be completed before another can start.
pub struct CalenderTaskDependencyEntity {
    /// Unique identifier for this dependency.
    pub id: i64,

    /// ID of the task that must complete first.
    pub blocking_task_id: i64,

    /// ID of the task that is waiting on the blocker.
    pub blocked_task_id: i64,
}
