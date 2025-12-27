use crate::ids::{ActorId, CandidateId};
use crate::time::Timestamp;

use super::vote_type::VoteType;

#[derive(Debug, Clone)]
pub struct Vote {
    /// The id of the one who voted
    pub actor_id: ActorId,

    /// The id of the item voting for
    pub candidate_id: CandidateId,

    /// accept, reject, or abstain
    pub vtype: VoteType,

    pub recorded_at: Timestamp,
}
