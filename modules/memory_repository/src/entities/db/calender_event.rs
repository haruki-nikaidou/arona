use time::{Date, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

pub struct CalenderEventEntity {
    pub id: i64,
    pub calender_id: Uuid,
    pub title: String,
    pub description: String,
    pub time: OffsetDateTime,
    pub repeat: CalenderEventRepeat,
    pub repeat_until: Option<Date>,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

pub enum CalenderEventRepeat {
    NoRepeat,
    EveryDay,
    EveryMonth,
    EveryWeekday,
}
