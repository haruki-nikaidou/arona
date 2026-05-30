use time::{Date, PrimitiveDateTime};
use uuid::Uuid;

pub struct CalenderDailyEventEntity {
    pub id: i64,
    pub calender_id: Uuid,
    pub title: String,
    pub description: String,
    pub date: Date,
    pub repeat: DailyEventRepeat,
    pub repeat_until: Option<Date>,
    pub created: PrimitiveDateTime,
    pub updated: PrimitiveDateTime,
}

pub enum DailyEventRepeat {
    NoRepeat,
    EveryMonth,
    EveryYear,
    EveryWeekday,
}
