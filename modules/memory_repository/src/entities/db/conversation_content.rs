//! Multimodal content blocks within conversation messages.

use kanau::processor::Processor;
use tracing::instrument;
use wakuwaku::sqlx::DatabaseProcessor;

/// A single content block within a conversation message.
///
/// Messages can contain multiple content blocks of different modalities
/// (text, images, audio, etc.). Each block is stored separately to support
/// efficient retrieval and multimodal search.
#[derive(Debug, Clone)]
pub struct ConversationContentEntity {
    /// Unique identifier for this content block.
    pub id: i64,

    /// ID of the parent [`ConversationMessageEntity`](super::conversation_message::ConversationMessageEntity).
    pub message_id: i64,

    /// Order of this block within the message (0-indexed).
    pub position: i32,

    /// The type of content in this block.
    pub modality: ContentModality,

    /// Text content (for [`ContentModality::Text`]).
    pub text: Option<String>,

    /// Reference to stored object (for non-text modalities).
    /// The DB enforces `octet_length(object_hash) = 32` via a CHECK constraint.
    pub object_hash: Option<Vec<u8>>,

    /// Image detail level hint (e.g., `low`, `high`, `auto`).
    pub image_detail: Option<String>,

    /// Audio encoding format (e.g., `mp3`, `wav`, `opus`).
    pub audio_format: Option<String>,
}

/// The type of content in a message block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "memory.content_modality")]
pub enum ContentModality {
    /// Plain text content.
    Text,
    /// Image content (stored in object storage).
    Image,
    /// Audio content (stored in object storage).
    Audio,
    /// Generic file attachment (stored in object storage).
    File,
    /// Video content (stored in object storage).
    Video,
}

/// Find a [`ConversationContentEntity`] by its bigserial primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindConversationContentById {
    pub id: i64,
}

impl Processor<FindConversationContentById> for DatabaseProcessor {
    type Output = Option<ConversationContentEntity>;
    type Error = sqlx::Error;

    #[instrument(skip_all, name = "SQL:FindConversationContentById", err, fields(id = input.id))]
    async fn process(
        &self,
        input: FindConversationContentById,
    ) -> Result<Option<ConversationContentEntity>, sqlx::Error> {
        sqlx::query_as!(
            ConversationContentEntity,
            r#"
            SELECT
                id,
                message_id,
                position,
                modality AS "modality: ContentModality",
                text,
                object_hash,
                image_detail,
                audio_format
            FROM memory.conversation_content
            WHERE id = $1
            LIMIT 1
            "#,
            input.id,
        )
        .fetch_optional(self.db())
        .await
    }
}