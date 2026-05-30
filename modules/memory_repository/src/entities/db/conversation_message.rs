//! Individual messages within conversations.

use time::PrimitiveDateTime;

/// A single message in a conversation thread.
///
/// Messages form a linked list via the `before` field, supporting branching
/// conversation histories (e.g., when the user regenerates a response).
pub struct ConversationMessageEntity {
    /// Unique identifier for this message.
    pub id: i64,

    /// ID of the parent [`ConversationEntity`](super::conversation::ConversationEntity).
    pub conversation_id: i64,

    /// ID of the preceding message (forms a linked list for branching support).
    pub before: Option<i64>,

    /// When this message was created.
    pub created_at: PrimitiveDateTime,

    /// Whether this message is on the currently active branch.
    pub is_current_branch: bool,
}

/// The role of a message sender in a conversation.
pub enum MessageRole {
    /// System instructions or context.
    System,
    /// Message from the human user.
    User,
    /// Response from the AI assistant.
    Assistant,
    /// Output from a tool invocation.
    Tool,
}
