//! Calendar containers for organizing events and tasks.

use kanau::processor::Processor;
use time::PrimitiveDateTime;
use tracing::instrument;
use uuid::Uuid;
use wakuwaku::sqlx::DatabaseProcessor;

/// A calendar that groups related events and tasks.
///
/// Calendars provide organizational structure and access control for
/// scheduling data. Multiple calendars can coexist (e.g., work, personal).
#[derive(Debug, Clone)]
pub struct CalenderEntity {
    /// Unique identifier for this calendar.
    pub id: Uuid,

    /// Display name for this calendar.
    pub name: String,

    /// Description of the calendar's purpose.
    pub description: String,

    /// When this calendar was created.
    pub created_at: PrimitiveDateTime,

    /// Permission scopes that can access this calendar.
    pub scopes: Vec<String>,
}

/// Find a [`CalenderEntity`] by its UUID primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindCalenderById {
    pub id: Uuid,
}

impl Processor<FindCalenderById> for DatabaseProcessor {
    type Output = Option<CalenderEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindCalenderById", err, fields(id = %input.id))]
    async fn process(&self, input: FindCalenderById) -> Result<Option<CalenderEntity>, sqlx::Error> {
        sqlx::query_as!(
            CalenderEntity,
            r#"
            SELECT id, name, description, created_at, scopes
            FROM memory.calender
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
