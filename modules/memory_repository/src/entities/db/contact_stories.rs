use time::PrimitiveDateTime;
use uuid::Uuid;

pub struct ContactStoryEntity {
    pub id: i64,
    pub identity: Uuid,
    pub story_name: String,
    pub story_summary: String,
    pub story_text: String,
    pub happened_at: PrimitiveDateTime,
    pub related_conversation: Option<i64>,
}

pub enum StoryType {
    RelationshipUpgrade,
    RelationshipDowngrade,
    FirstMeeting,
    ImpressionChanged,
    Other,
}
