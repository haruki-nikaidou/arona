//! Platform-specific contact records.

use crate::entities::db::contact_identity::ContactIdentityEntity;
use time::PrimitiveDateTime;

/// A contact as they appear on a specific platform.
///
/// The same person may have multiple `ContactEntity` records (one per platform),
/// but they share a single [`ContactIdentityEntity`] that represents the
/// real-world identity Isla has recognized.
pub struct ContactEntity {
    /// Unique identifier for this contact record.
    pub id: i64,

    /// The name shown for this contact on the platform.
    pub display_name: String,

    /// Platform-specific user identifier.
    pub user_id: String,

    /// Platform name (e.g., `discord`, `telegram`, `slack`).
    pub platform: String,

    /// The cross-platform identity this contact is linked to.
    pub identity: ContactIdentityEntity,

    /// When this contact was first seen.
    pub created_at: PrimitiveDateTime,

    /// When this contact record was last updated.
    pub updated_at: PrimitiveDateTime,
}
