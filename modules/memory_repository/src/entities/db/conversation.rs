//! Conversation sessions and their summaries.

/// A conversation session between the user and the agent.
///
/// Conversations are the top-level container for a series of messages.
/// They maintain summaries for quick context retrieval without loading
/// the full message history.
pub struct ConversationEntity {
    /// Unique identifier for this conversation.
    pub id: i64,

    /// AI-generated summary of how the conversation started.
    pub opening_summary: String,

    /// AI-generated summary of the conversation outcome (set when conversation ends).
    pub closing_summary: Option<String>,

    /// Unix timestamp when the conversation started.
    pub created_at: i64,

    /// Unix timestamp of the last activity in this conversation.
    pub updated_at: i64,
}
