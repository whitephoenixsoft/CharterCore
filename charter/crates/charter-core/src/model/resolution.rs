use super::resolution_state::ResolutionState;
use crate::ids::{AreaId, ObjectHash, ResolutionId, SessionId};
use crate::time::Timestamp;

/// Immutable decision record produced by a session
#[derive(Debug, Clone)]
pub struct Resolution {
    /// Engine-opaque identifier
    pub id: ResolutionId,

    /// Deterministic has of this resolutions canonical from
    pub object_hash: ObjectHash,

    /// Area this resolution belongs to
    pub area_id: AreaId,

    /// Session in which this resolution was accepted
    pub accepted_in_session: SessionId,

    /// Lifecycle state (Active, Superceded, etc)
    pub state: ResolutionState,

    /// Timestamp of acceptance
    pub accepted_at: Timestamp,

    /// Authority resolution active at acceptance time
    pub authority_resolution_id: ResolutionId,

    /// Scope resolution active at acceptance time
    pub scope_resolution_id: ResolutionId,

    /// Additional scopes explcitly referenced by the session
    pub referenced_scope_ids: Vec<ResolutionId>,

    /// Resolution that this resolution explicitly supercedes (if any)
    pub supercedes: Option<ResolutionId>,

    /// Opaque resolution content (engine does not interpret)
    pub content: ResolutionContent,
}

/// Opage uninterpreted resolution content
#[derive(Debug, Clone)]
pub struct ResolutionContent {
    /// Rqw payload (CLI / UI / App defind )
    pub text: String,
}
