use time::PrimitiveDateTime;
use uuid::Uuid;

pub struct CalenderEntity {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: PrimitiveDateTime,
    pub scopes: Vec<String>,
}
