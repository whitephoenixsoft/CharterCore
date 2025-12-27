use crate::ids::{AreaId, ResolutionId, ObjectHash};
use crate::time::Timestamp;

/// Engine-owned Area record.
/// Areas are hard governance boundaries.
pub struct Area {
    pub id: AreaId,
    pub object_hash: ObjectHash,

    /// Creation metadata (audit only)
    pub created_at: Timestamp,

    /// Optional freeform notes/name
    pub notes: Option<String>,

    /// Active authority resolution. (required for initialization)
    pub active_authority: Option<ResolutionId>,

    /// Active Scope resolution. (required for initialization)
    pub active_scope: Option<ResolutionId>,
}
