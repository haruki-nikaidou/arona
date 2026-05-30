use time::PrimitiveDateTime;
use uuid::Uuid;

pub struct ContactIdentityEntity {
    pub id: Uuid,
    pub identify_name: String,
    pub description: String,
    pub relationship: Relationship,
    pub first_meet_at: PrimitiveDateTime,
    pub relationship_updated_at: PrimitiveDateTime,
}

pub enum Relationship {
    Stranger,
    Master,
    Acquaintance,
    Dude,
    Ignored,
}
