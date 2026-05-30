//! Individual messages within conversations.

use kanau::processor::Processor;
use time::PrimitiveDateTime;
use tracing::instrument;
use wakuwaku::sqlx::DatabaseProcessor;

/// A single message in a conversation thread.
///
/// Messages form a linked list via the `before` field, supporting branching
/// conversation histories (e.g., when the user regenerates a response).
#[derive(Debug, Clone)]
pub struct ConversationMessageEntity {
    /// Unique identifier for this message.
    pub id: i64,

    /// ID of the parent [`ConversationEntity`](super::conversation::ConversationEntity).
    pub conversation_id: i64,

    /// ID of the preceding message (forms a linked list for branching support).
    pub before: Option<i64>,

    /// Role of the sender of this message.
    pub role: MessageRole,

    /// When this message was created.
    pub created_at: PrimitiveDateTime,

    /// Whether this message is on the currently active branch.
    pub is_current_branch: bool,
}

/// The role of a message sender in a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "memory.message_role")]
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

/// Find a [`ConversationMessageEntity`] by its bigserial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindConversationMessageById {
    pub id: i64,
}

impl Processor<FindConversationMessageById> for DatabaseProcessor {
    type Output = Option<ConversationMessageEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindConversationMessageById", err, fields(id = input.id))]
    async fn process(
        &self,
        input: FindConversationMessageById,
    ) -> Result<Option<ConversationMessageEntity>, sqlx::Error> {
        sqlx::query_as!(
            ConversationMessageEntity,
            r#"
            SELECT
                id,
                conversation_id,
                before,
                role AS "role: MessageRole",
                created_at,
                is_current_branch
            FROM memory.conversation_message
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
