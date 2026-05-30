//! Multimodal content blocks within conversation messages.

/// A single content block within a conversation message.
///
/// Messages can contain multiple content blocks of different modalities
/// (text, images, audio, etc.). Each block is stored separately to support
/// efficient retrieval and multimodal search.
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
    pub object_hash: Option<[u8; 32]>,

    /// Image detail level hint (e.g., `low`, `high`, `auto`).
    pub image_detail: Option<String>,

    /// Audio encoding format (e.g., `mp3`, `wav`, `opus`).
    pub audio_format: Option<String>,
}

/// The type of content in a message block.
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