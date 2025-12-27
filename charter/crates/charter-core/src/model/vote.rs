use crate::ids::{ActorId, CandidateId};
use crate::time::Timestamp;

/// Immutable voting record on the stance for a candidate
#[derive(Debug, Clone)]
pub struct Vote {
    /// The id of the one who voted
    pub actor_id: ActorId,

    /// The id of the item voting for
    pub candidate_id: CandidateId,

    /// accept, reject, or abstain
    pub stance: VoteType,

    pub recorded_at: Timestamp,
}

/// Types of explicit voting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoteType {
    Accept,
    Reject,
    Abstain,
}
