use time::PrimitiveDateTime;
use uuid::Uuid;
use crate::entities::db::accounts::AccountRole;

#[derive(Debug, Clone)]
/// Invitation for new members to join the team.
///
/// - Schema: `auth`
/// - Table Name: `invitation`
pub struct InvitationEntity {
    /// Primary key
    pub token: Uuid,

    pub created_at: PrimitiveDateTime,

    /// All invitation must have an expiry time, after which the token is invalid.
    pub expire_at: PrimitiveDateTime,

    /// The number of user can accept this invitation. If `None`, there is no limit.
    pub max_accept_count: Option<u32>,

    /// The role assigned to user after registration
    pub role: AccountRole,

    /// Foreign key to [AccountEntity](super::accounts::AccountEntity)
    pub send_by: Uuid
}
