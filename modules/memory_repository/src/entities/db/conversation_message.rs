//! Individual messages within conversations.

use kanau::processor::Processor;
use time::PrimitiveDateTime;
use tracing::instrument;
use wakuwaku::sqlx::DatabaseProcessor;

use super::conversation_content::ContentModality;

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

/// Insert a new message into a conversation.
#[derive(Debug, Clone, Copy)]
pub struct CreateConversationMessage {
    pub conversation_id: i64,
    pub before: Option<i64>,
    pub role: MessageRole,
}

impl Processor<CreateConversationMessage> for DatabaseProcessor {
    type Output = i64;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:CreateConversationMessage", err,
        fields(conversation_id = input.conversation_id))]
    async fn process(&self, input: CreateConversationMessage) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar!(
            r#"
            INSERT INTO memory.conversation_message (conversation_id, before, role)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            input.conversation_id,
            input.before,
            input.role as MessageRole,
        )
        .fetch_one(self.db())
        .await
    }
}

/// Set the branch flag on a message (used when switching active branches).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetConversationMessageBranch {
    pub id: i64,
    pub is_current_branch: bool,
}

impl Processor<SetConversationMessageBranch> for DatabaseProcessor {
    type Output = bool;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:SetConversationMessageBranch", err, fields(id = input.id))]
    async fn process(&self, input: SetConversationMessageBranch) -> Result<bool, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            UPDATE memory.conversation_message
            SET is_current_branch = $2
            WHERE id = $1
            "#,
            input.id,
            input.is_current_branch,
        )
        .execute(self.db())
        .await?
        .rows_affected();
        Ok(rows > 0)
    }
}

/// Delete a message (cascades to its content blocks).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteConversationMessage {
    pub id: i64,
}

impl Processor<DeleteConversationMessage> for DatabaseProcessor {
    type Output = bool;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:DeleteConversationMessage", err, fields(id = input.id))]
    async fn process(&self, input: DeleteConversationMessage) -> Result<bool, sqlx::Error> {
        let rows = sqlx::query!(
            "DELETE FROM memory.conversation_message WHERE id = $1",
            input.id,
        )
        .execute(self.db())
        .await?
        .rows_affected();
        Ok(rows > 0)
    }
}

/// Paginated list of messages in a conversation ordered by creation time.
///
/// When `only_current_branch` is `true` only messages on the active branch are returned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListConversationMessages {
    pub conversation_id: i64,
    pub only_current_branch: bool,
    pub limit: i64,
    pub offset: i64,
}

impl Processor<ListConversationMessages> for DatabaseProcessor {
    type Output = Vec<ConversationMessageEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:ListConversationMessages", err,
        fields(conversation_id = input.conversation_id))]
    async fn process(
        &self,
        input: ListConversationMessages,
    ) -> Result<Vec<ConversationMessageEntity>, sqlx::Error> {
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
            WHERE conversation_id = $1
              AND (NOT $2 OR is_current_branch)
            ORDER BY created_at ASC
            LIMIT $3 OFFSET $4
            "#,
            input.conversation_id,
            input.only_current_branch,
            input.limit,
            input.offset,
        )
        .fetch_all(self.db())
        .await
    }
}

/// A single content block to be inserted as part of [`AppendMessageTx`].
#[derive(Debug, Clone)]
pub struct NewContent {
    pub position: i32,
    pub modality: ContentModality,
    pub text: Option<String>,
    pub object_hash: Option<Vec<u8>>,
    pub image_detail: Option<String>,
    pub audio_format: Option<String>,
}

/// Atomically insert a message and all its content blocks in one transaction.
///
/// Also touches the parent conversation's `updated_at` timestamp within the same
/// transaction to keep everything consistent.
#[derive(Debug, Clone)]
pub struct AppendMessageTx {
    pub conversation_id: i64,
    pub before: Option<i64>,
    pub role: MessageRole,
    pub contents: Vec<NewContent>,
    /// Unix timestamp to write to `conversation.updated_at`.
    pub conversation_updated_at: i64,
}

impl Processor<AppendMessageTx> for DatabaseProcessor {
    type Output = i64;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL-Transaction:AppendMessageTx", err,
        fields(conversation_id = input.conversation_id))]
    async fn process(&self, input: AppendMessageTx) -> Result<i64, sqlx::Error> {
        let mut tx = self.db().begin().await?;

        let message_id: i64 = sqlx::query_scalar!(
            r#"
            INSERT INTO memory.conversation_message (conversation_id, before, role)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            input.conversation_id,
            input.before,
            input.role as MessageRole,
        )
        .fetch_one(&mut *tx)
        .await?;

        for content in &input.contents {
            sqlx::query!(
                r#"
                INSERT INTO memory.conversation_content
                    (message_id, position, modality, text, object_hash, image_detail, audio_format)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                message_id,
                content.position,
                content.modality as ContentModality,
                content.text,
                content.object_hash,
                content.image_detail,
                content.audio_format,
            )
            .execute(&mut *tx)
            .await?;
        }

        sqlx::query!(
            "UPDATE memory.conversation SET updated_at = $2 WHERE id = $1",
            input.conversation_id,
            input.conversation_updated_at,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(message_id)
    }
}
