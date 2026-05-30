//! Calendar containers for organizing events and tasks.

use time::PrimitiveDateTime;
use uuid::Uuid;

/// A calendar that groups related events and tasks.
///
/// Calendars provide organizational structure and access control for
/// scheduling data. Multiple calendars can coexist (e.g., work, personal).
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
