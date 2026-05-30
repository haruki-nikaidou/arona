use time::{Date, PrimitiveDateTime};

pub struct DiaryEntity {
    pub id: i64,
    pub title: String,
    pub date: Date,
    pub summary: String,
    pub content: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}