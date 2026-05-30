//! Conversation sessions and their summaries.

use kanau::processor::Processor;
use tracing::instrument;
use wakuwaku::sqlx::DatabaseProcessor;

/// A conversation session between the user and the agent.
///
/// Conversations are the top-level container for a series of messages.
/// They maintain summaries for quick context retrieval without loading
/// the full message history.
#[derive(Debug, Clone)]
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

/// Find a [`ConversationEntity`] by its bigserial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindConversationById {
    pub id: i64,
}

impl Processor<FindConversationById> for DatabaseProcessor {
    type Output = Option<ConversationEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindConversationById", err, fields(id = input.id))]
    async fn process(
        &self,
        input: FindConversationById,
    ) -> Result<Option<ConversationEntity>, sqlx::Error> {
        sqlx::query_as!(
            ConversationEntity,
            r#"
            SELECT id, opening_summary, closing_summary, created_at, updated_at
            FROM memory.conversation
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}
