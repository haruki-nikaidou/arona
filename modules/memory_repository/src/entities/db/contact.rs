use time::PrimitiveDateTime;
use crate::entities::db::contact_identity::ContactIdentityEntity;

pub struct ContactEntity {
    pub id: i64,
    pub display_name: String,
    pub user_id: String,
    pub platform: String,
    pub identity: ContactIdentityEntity,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}
