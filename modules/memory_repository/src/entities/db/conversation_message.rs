use time::PrimitiveDateTime;

pub struct ConversationMessageEntity {
    pub id: i64,
    pub conversation_id: i64,
    pub before: Option<i64>,
    pub created_at: PrimitiveDateTime,
    pub is_current_branch: bool,
}

pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}
