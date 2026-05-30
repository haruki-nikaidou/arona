use time::PrimitiveDateTime;
use uuid::Uuid;

pub struct CalenderTaskEntity {
    pub id: i64,
    pub calendar_id: Uuid,
    pub title: String,
    pub description: String,
    pub start_at: PrimitiveDateTime,
    pub deadline: PrimitiveDateTime,
    pub status_update_at: PrimitiveDateTime,
}

pub enum CalenderTaskStatus {
    Pending,
    Doing,
    Finished,
}

pub struct CalenderTaskDependencyEntity {
    pub id: i64,
    pub blocking_task_id: i64,
    pub blocked_task_id: i64,
}
