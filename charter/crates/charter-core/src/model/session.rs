use super::candidate::Candidate;
use super::vote::Vote;
use crate::ids::{AreaId, ObjectHash, ResolutionId, SessionId};
use crate::time::Timestamp;

/// record saving the instance of a addition/change to a resolution
#[derive(Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub area_id: AreaId,

    pub problem_statement: String,

    // Frozen content
    pub authority_resolution_id: ResolutionId,
    pub authority_rule: AuthorityRule,
    pub scope_resolution_id: ResolutionId,
    pub referenced_scope_ids: Vec<ResolutionId>,

    /// Optional explicit supersession target
    pub preceding_resolution_id: Option<ResolutionId>,

    // Lifecyle fields
    pub state: SessionState,
    pub started_at: Timestamp,
    pub closed_at: Option<String>,

    // Session contents
    pub candidates: Vec<Candidate>,
    pub votes: Vec<Vote>,

    /// Deterministice integrity check
    pub object_hash: ObjectHash,
}

/// The state of a session
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Active,
    Paused,
    Blocked,
    Closed,
}

/// The rule governing session voting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorityRule {
    SoleActor,
    UnanimousPresent,
    MajorityPresent,
}
