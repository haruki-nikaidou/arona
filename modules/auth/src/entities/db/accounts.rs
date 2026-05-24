use time::PrimitiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
/// Account for owner and trusted members.
/// 
/// - Schema: `auth`
/// - Table Name: `account`
pub struct AccountEntity {
    /// Primary key
    pub id: Uuid,
    
    /// Unique username
    pub username: String,
    
    /// Argon2 hashed password
    pub password: String,
    
    /// The time user registered. Readonly after creation.
    pub registered_at: PrimitiveDateTime,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "auth.account_status")]
pub enum AccountRole {
    Owner,
    Member,
}
