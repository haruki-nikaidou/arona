//! Task management within calendars.

use kanau::processor::Processor;
use time::PrimitiveDateTime;
use tracing::instrument;
use uuid::Uuid;
use wakuwaku::sqlx::DatabaseProcessor;

/// A task with a deadline, tracked within a calendar.
///
/// Tasks differ from events in that they have a duration window (start to deadline)
/// and progress tracking, rather than occurring at a specific moment.
#[derive(Debug, Clone)]
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

    /// Current progress state of the task.
    pub status: CalenderTaskStatus,

    /// When the task status was last changed.
    pub status_update_at: PrimitiveDateTime,
}

/// Progress state of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "memory.calender_task_status")]
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
#[derive(Debug, Clone)]
pub struct CalenderTaskDependencyEntity {
    /// Unique identifier for this dependency.
    pub id: i64,

    /// ID of the task that must complete first.
    pub blocking_task_id: i64,

    /// ID of the task that is waiting on the blocker.
    pub blocked_task_id: i64,
}

/// Find a [`CalenderTaskEntity`] by its bigserial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindCalenderTaskById {
    pub id: i64,
}

impl Processor<FindCalenderTaskById> for DatabaseProcessor {
    type Output = Option<CalenderTaskEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindCalenderTaskById", err, fields(id = input.id))]
    async fn process(
        &self,
        input: FindCalenderTaskById,
    ) -> Result<Option<CalenderTaskEntity>, sqlx::Error> {
        sqlx::query_as!(
            CalenderTaskEntity,
            r#"
            SELECT
                id,
                calendar_id,
                title,
                description,
                start_at,
                deadline,
                status AS "status: CalenderTaskStatus",
                status_update_at
            FROM memory.calender_task
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}

/// Find a [`CalenderTaskDependencyEntity`] by its bigserial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindCalenderTaskDependencyById {
    pub id: i64,
}

impl Processor<FindCalenderTaskDependencyById> for DatabaseProcessor {
    type Output = Option<CalenderTaskDependencyEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindCalenderTaskDependencyById", err, fields(id = input.id))]
    async fn process(
        &self,
        input: FindCalenderTaskDependencyById,
    ) -> Result<Option<CalenderTaskDependencyEntity>, sqlx::Error> {
        sqlx::query_as!(
            CalenderTaskDependencyEntity,
            r#"
            SELECT id, blocking_task_id, blocked_task_id
            FROM memory.calender_task_dependency
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
