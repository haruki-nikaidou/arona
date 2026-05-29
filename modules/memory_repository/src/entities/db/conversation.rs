pub struct ConversationEntity {
    pub id: i64,
    pub opening_summary: String,
    pub closing_summary: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
