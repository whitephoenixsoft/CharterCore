use crate::domain::enums::*;
use crate::domain::ids::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrossAreaReference {
    pub external_area_id: String,
    pub external_area_label: String,
    pub external_resolution_id: Option<String>,
    pub external_resolution_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Participant {
    pub participant_id: ParticipantId,
    pub session_id: SessionId,
    pub area_id: AreaId,
    pub round_index: u32,
    pub display_name: String,
    pub annotation: Option<String>,
    pub created_at: Option<String>,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CandidatePayload {
    AdoptResolution {
        resolution_content: String,
    },
    SupersedeResolutions {
        resolution_content: String,
        supersedes_resolution_ids: Vec<ResolutionId>,
    },
    RetireResolution {
        target_resolution_id: ResolutionId,
    },
}

impl CandidatePayload {
    pub fn action_type(&self) -> CandidateActionType {
        match self {
            Self::AdoptResolution { .. } => CandidateActionType::AdoptResolution,
            Self::SupersedeResolutions { .. } => CandidateActionType::SupersedeResolutions,
            Self::RetireResolution { .. } => CandidateActionType::RetireResolution,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    pub candidate_id: CandidateId,
    pub session_id: SessionId,
    pub area_id: AreaId,
    pub round_index: u32,
    pub candidate_payload: CandidatePayload,
    pub annotation: Option<String>,
    pub created_at: Option<String>,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    pub constraint_id: ConstraintId,
    pub session_id: SessionId,
    pub area_id: AreaId,
    pub round_index: u32,
    pub constraint_type: String,
    pub constraint_parameters: String,
    pub annotation: Option<String>,
    pub created_at: Option<String>,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    pub vote_id: VoteId,
    pub session_id: SessionId,
    pub area_id: AreaId,
    pub round_index: u32,
    pub participant_id: ParticipantId,
    pub candidate_id: CandidateId,
    pub stance: Stance,
    pub annotation: Option<String>,
    pub created_at: Option<String>,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub session_id: SessionId,
    pub area_id: AreaId,
    pub session_type: SessionType,
    pub authority_id: Option<ResolutionId>,
    pub scope_id: Option<ResolutionId>,
    pub phase: SessionPhase,
    pub state: SessionState,
    pub round_index: u32,
    pub participants: Vec<Participant>,
    pub candidates: Vec<Candidate>,
    pub constraints: Vec<Constraint>,
    pub votes: Vec<Vote>,
    pub internal_resolution_references: Vec<ResolutionId>,
    pub cross_area_references: Vec<CrossAreaReference>,
    pub terminal_receipt_id: Option<ReceiptId>,
    pub annotation: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolution {
    pub resolution_id: ResolutionId,
    pub area_id: AreaId,
    pub originating_session_id: SessionId,
    pub authority_snapshot_id: Option<ResolutionId>,
    pub scope_snapshot_id: Option<ResolutionId>,
    pub accepted_candidate_id: CandidateId,
    pub kind: ResolutionKind,
    pub engine_version: String,
    pub spec_set_hash: String,
    pub state: ResolutionState,
    pub superseded_by: Option<ResolutionId>,
    pub internal_resolution_references: Vec<ResolutionId>,
    pub cross_area_references: Vec<CrossAreaReference>,
    pub annotation: Option<String>,
    pub created_at: Option<String>,
    pub schema_version: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoundSnapshot {
    pub round_index: u32,
    pub round_state: RoundState,
    pub participant_set: Vec<Participant>,
    pub candidate_set: Vec<Candidate>,
    pub constraint_set: Vec<Constraint>,
    pub vote_set: Vec<Vote>,
    pub internal_resolution_references: Vec<ResolutionId>,
    pub cross_area_references: Vec<CrossAreaReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiptBody {
    Legitimacy { resolution_id: ResolutionId },
    Exploration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    pub receipt_id: ReceiptId,
    pub session_id: SessionId,
    pub body: ReceiptBody,
    pub area_id: AreaId,
    pub engine_version: String,
    pub spec_set_hash: String,
    pub authority_snapshot_id: Option<ResolutionId>,
    pub scope_snapshot_id: Option<ResolutionId>,
    pub problem_statement: Option<String>,
    pub rounds: Vec<RoundSnapshot>,
    pub final_round_index: u32,
    pub session_state_at_close: SessionState,
    pub acceptance_result: AcceptanceResult,
    pub annotation: Option<String>,
    pub created_at: Option<String>,
    pub hash_algorithm: String,
    pub content_hash: String,
    pub schema_version: u32,
}

impl Receipt {
    pub fn receipt_type(&self) -> ReceiptType {
        match self.body {
            ReceiptBody::Legitimacy { .. } => ReceiptType::Legitimacy,
            ReceiptBody::Exploration => ReceiptType::Exploration,
        }
    }

    pub fn resolution_id(&self) -> Option<&ResolutionId> {
        match &self.body {
            ReceiptBody::Legitimacy { resolution_id } => Some(resolution_id),
            ReceiptBody::Exploration => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AreaGraph {
    pub area_id: Option<AreaId>,
    pub sessions: Vec<Session>,
    pub resolutions: Vec<Resolution>,
    pub receipts: Vec<Receipt>,
}