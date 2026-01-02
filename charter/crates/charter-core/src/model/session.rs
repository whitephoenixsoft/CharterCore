/*
use super::candidate::Candidate;
use super::vote::Vote;
use crate::ids::{ObjectHash, ExternalId, ActorId};
use crate::time::Timestamp;

/// record saving the instance of a addition/change to a resolution
#[derive(Debug, Clone)]
pub struct Session {
    pub label: ExternalId,
    pub area_id: ObjectHash,

    pub problem_statement: String,

    // Frozen content
    pub authority_resolution_id: ObjectHash,
    pub authority_rule: AuthorityRule,
    pub scope_resolution_id: ObjectHash,
    pub referenced_scope_ids: Vec<ObjectHash>,

    /// Optional explicit supersession target
    pub preceding_resolution_id: Option<ObjectHash>,

    // Lifecyle fields
    pub state: SessionState,
    pub started_at: Timestamp,
    pub closed_at: Option<String>,

    // Session contents
    pub participants: Vec<ActorId>,
    pub candidates: Vec<Candidate>,
    pub votes: Vec<Vote>,

    /// Deterministice integrity check
    pub object_hash: ObjectHash,
}

/// The state of a session
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    Active,
    Paused,
    Blocked { reason: String },
    Closed,
}

/// The rule governing session voting
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorityRule {
    SoleActor { actor: ActorId },
    UnanimousPresent,
    MajorityPresent,
}
*/
