#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResolutionState {
    Active,
    Superseded,
    OnHold,
    Retired,
}

impl ResolutionState {
    pub fn is_active_structural_candidate(self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SessionPhase {
    PreStance,
    Voting,
    Terminal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SessionState {
    Active,
    Paused,
    BlockTemporary,
    BlockPermanent,
    Accepted,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SessionType {
    Authority,
    Scope,
    Regular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CandidateActionType {
    AdoptResolution,
    SupersedeResolutions,
    RetireResolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReceiptType {
    Legitimacy,
    Exploration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AcceptanceResult {
    Success,
    Abandoned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RoundState {
    Completed,
    FinalAccepted,
    Abandoned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stance {
    Accept,
    Reject,
    Abstain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResolutionKind {
    Authority,
    Scope,
    Regular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CrossAreaRelationship {
    DerivedFrom,
    Affects,
    AffectedBy,
    Related,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReversibilityIntent {
    Reversible,
    ConditionallyReversible,
    Irreversible,
}