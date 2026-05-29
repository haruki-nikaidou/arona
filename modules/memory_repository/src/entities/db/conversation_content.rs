pub struct ConversationContentEntity {
    pub id: i64,
    pub message_id: i64,
    pub position: i32,
    pub modality: ContentModality,
    pub text: Option<String>,
    pub object_hash: Option<[u8; 32]>,
    pub image_detail: Option<String>,
    pub audio_format: Option<String>,
}

pub enum ContentModality {
    Text,
    Image,
    Audio,
    File,
    Video
}