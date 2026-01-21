use super::ids::{AreaId, ResolutionId};
use crate::time::Timestamp;

/// Engine-owned Area record.
/// Areas are hard governance boundaries.
pub struct Area {
    /// Unique hash that never changes
    pub area_id: AreaId,

    /// Short label for reference; mutabke
    pub label: String,

    /// Area full name; mutable
    pub name: String,

    /// Rationalle around the name (user memory aid); mutable
    pub annotation: Option<String>,

    /// Creation metadata (audit only)
    pub created_at: Timestamp,

    /// Active authority resolution. (required for initialization); mutable
    pub active_authority: Option<ResolutionId>,

    /// Active Scope resolution. (required for initialization); mutable
    pub active_scope: Option<ResolutionId>,
}


