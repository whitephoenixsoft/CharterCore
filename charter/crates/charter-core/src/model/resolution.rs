/*
use crate::ids::{ObjectHash, ExternalId, ActorId};
use crate::time::Timestamp;

/// Immutable decision record produced by a session
#[derive(Debug, Clone)]
pub struct Resolution {
    /// Engine-opaque identifier
    pub label: ExternalId,

    /// Deterministic has of this resolutions canonical from
    pub object_hash: ObjectHash,

    /// Area this resolution belongs to
    pub area_id: ObjectHash,

    /// Session in which this resolution was accepted
    pub accepted_in_session: ObjectHash,

    /// Lifecycle state (Active, Superceded, etc)
    pub state: ResolutionState,

    /// Type of resolution
    pub resolution_type: ResolutionType,

    /// Timestamp of acceptance
    pub accepted_at: Timestamp,

    /// Authority resolution active at acceptance time
    pub authority_resolution_id: ObjectHash,

    /// Scope resolution active at acceptance time
    pub scope_resolution_id: ObjectHash,

    /// Participants who were present at vote time
    pub participants: Vec<ActorId>,

    /// Additional scopes explcitly referenced by the session
    pub referenced_scope_ids: Vec<ObjectHash>,

    /// Resolution that this resolution explicitly supercedes (if any)
    pub supercedes: Option<ObjectHash>,

    /// Opaque resolution content (engine does not interpret)
    pub content: ResolutionContent,
}

/// Opaque uninterpreted resolution content
#[derive(Debug, Clone)]
pub struct ResolutionContent {
    /// Rqw payload (CLI / UI / App defined )
    pub text: String,
}

/// Possible states of the resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionState {
    Active,
    UnderReview,
    Superseded,
    Retired,
}

/// Type of resolution (affects the behavior)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionType {
   General,
   Authority,
   Scope,
}

/// This makes resolutions auditable and explicit
#[derive(Debug, Clone)]
pub struct ResolutionTransition {
   pub resolution_id: ObjectHash,
   pub from: ResolutionState,
   pub to: ResolutionState,
   pub occurred_at: Timestamp,
   pub session_id: Option<ObjectHash>, // can be marked under review without a session
} 
*/
