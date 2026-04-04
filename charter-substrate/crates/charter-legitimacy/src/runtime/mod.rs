#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateDisposition {
    Eligible,
    BlockedTemporary,
    BlockedPermanent,
    Invalid,
}