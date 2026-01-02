use super::ids::AreaId;
use crate::time::Timestamp;

/*
/// Engine-owned Area record.
/// Areas are hard governance boundaries.
pub struct Area {
    pub object_hash: ObjectHash,
    pub label: ExternalId,

    /// Creation metadata (audit only)
    pub created_at: Timestamp,

    /// Active authority resolution. (required for initialization)
    pub active_authority: Option<ObjectHash>,

    /// Active Scope resolution. (required for initialization)
    pub active_scope: Option<ObjectHash>,
}
*/

pub struct AreaRoot {
    pub area_id: AreaId,
    pub created_at: Timestamp,
}
