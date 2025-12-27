use crate::ids::{AreaId, ResolutionId};
use crate::time::Timestamp;

/// Engine-owned Area record.
/// Areas are hard governance boundaries.
pub struct Area {
    pub id: AreaId,

    /// Creation metadata (audit only)
    pub created_at: Timestamp,

    /// Optional freeform notes/name
    pub notes: Option<String>,

    /// Active authority resolution. (required for initialization)
    pub active_authority: Option<ResolutionId>,

    /// Active Scope resolution. (required for initialization)
    pub active_scope: Option<ResolutionId>,
}
