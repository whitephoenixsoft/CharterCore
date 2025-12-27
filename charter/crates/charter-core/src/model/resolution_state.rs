#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionState {
    Active,
    UnderReview,
    Superseded,
    Retired,
}
