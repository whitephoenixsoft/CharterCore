use std::collections::{BTreeMap, BTreeSet};

use crate::compiler::CompiledState;
use crate::domain::{
    CandidateId, CandidatePayload, ParticipantId, SessionId, SessionPhase, SessionState, Stance,
    Vote,
};
use crate::error::{
    ErrorCode, ErrorEntry, EvaluationOutcome, EvaluationReport, EvaluationReportBuilder,
    ReportPhase,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateDisposition {
    Eligible,
    BlockedTemporary,
    BlockedPermanent,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateEvaluation {
    pub candidate_id: CandidateId,
    pub disposition: CandidateDisposition,
    pub reasons: Vec<String>,
    pub accept_votes: usize,
    pub reject_votes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct EffectiveParticipantVotes {
    effective_accept: Option<CandidateId>,
    effective_rejects: BTreeSet<CandidateId>,
}

fn derive_effective_votes(votes: &[Vote]) -> BTreeMap<ParticipantId, EffectiveParticipantVotes> {
    let mut by_participant: BTreeMap<ParticipantId, EffectiveParticipantVotes> = BTreeMap::new();

    for vote in votes {
        let entry = by_participant
            .entry(vote.participant_id.clone())
            .or_default();

        match vote.stance {
            Stance::Accept => {
                entry.effective_accept = Some(vote.candidate_id.clone());
                entry.effective_rejects.remove(&vote.candidate_id);
            }
            Stance::Reject => {
                if entry.effective_accept.as_ref() == Some(&vote.candidate_id) {
                    entry.effective_accept = None;
                }
                entry.effective_rejects.insert(vote.candidate_id.clone());
            }
        }
    }

    by_participant
}

pub fn evaluate_session(state: &CompiledState, session_id: &SessionId) -> EvaluationReport {
    let Some(session) = state.sessions.get(session_id) else {
        return EvaluationReportBuilder::new(
            "evaluate_session",
            "session",
            Some(session_id.as_str()),
        )
        .error(ErrorEntry::new(
            ReportPhase::StructuralValidation,
            ErrorCode::MissingReference,
            vec![format!("session:{}", session_id.as_str())],
        ))
        .finish(false);
    };

    let mut errors = Vec::new();

    if matches!(session.state, SessionState::Accepted | SessionState::Closed) {
        errors.push(ErrorEntry::new(
            ReportPhase::SessionStateValidation,
            ErrorCode::SessionTerminalImmutable,
            vec![format!("session:{}", session.session_id.as_str())],
        ));
    }

    if session.phase == SessionPhase::PreStance && !session.votes.is_empty() {
        errors.push(ErrorEntry::new(
            ReportPhase::SessionStateValidation,
            ErrorCode::InvalidStateCombination,
            vec![format!("session:{}", session.session_id.as_str())],
        ));
    }

    if session.candidates.is_empty() {
        errors.push(ErrorEntry::new(
            ReportPhase::CandidateValidation,
            ErrorCode::NoEligibleCandidates,
            vec![format!("session:{}", session.session_id.as_str())],
        ));
    }

    let participant_ids = session
        .participants
        .iter()
        .map(|p| p.participant_id.as_str().to_string())
        .collect::<BTreeSet<_>>();

    let candidate_ids = session
        .candidates
        .iter()
        .map(|c| c.candidate_id.as_str().to_string())
        .collect::<BTreeSet<_>>();

    for candidate in &session.candidates {
        if candidate.round_index != session.round_index {
            errors.push(ErrorEntry::new(
                ReportPhase::CandidateValidation,
                ErrorCode::InvalidParticipantEpoch,
                vec![format!("candidate:{}", candidate.candidate_id.as_str())],
            ));
        }
    }

    for vote in &session.votes {
        if vote.round_index != session.round_index {
            errors.push(ErrorEntry::new(
                ReportPhase::VoteValidation,
                ErrorCode::InvalidParticipantEpoch,
                vec![format!("vote:{}", vote.vote_id.as_str())],
            ));
        }

        if !participant_ids.contains(vote.participant_id.as_str()) {
            errors.push(ErrorEntry::new(
                    ReportPhase::ParticipantValidation,
                    ErrorCode::ParticipantNotFound,
                    vec![
                        format!("vote:{}", vote.vote_id.as_str()),
                        format!("participant:{}", vote.participant_id.as_str()),
                    ],
                ));
        }

        if !candidate_ids.contains(vote.candidate_id.as_str()) {
            errors.push(ErrorEntry::new(
                ReportPhase::VoteValidation,
                ErrorCode::MissingReference,
                vec![
                    format!("vote:{}", vote.vote_id.as_str()),
                    format!("candidate:{}", vote.candidate_id.as_str()),
                ],
            ));
        }
    }

    errors.sort_by(|a, b| match a.error_code.cmp(&b.error_code) {
        std::cmp::Ordering::Equal => a.related_objects.cmp(&b.related_objects),
        other => other,
    });
    
    if errors.is_empty() {
        EvaluationReportBuilder::new(
            "evaluate_session",
            "session",
            Some(session_id.as_str()),
        )
        .finish(false)
    } else {
        let mut builder = EvaluationReportBuilder::new(
            "evaluate_session",
            "session",
            Some(session_id.as_str()),
        );
    
        for error in errors {
            builder.push_error(error);
        }
    
        builder.finish(false)
    }
}

pub fn evaluate_candidates_for_session(
    state: &CompiledState,
    session_id: &SessionId,
) -> Result<Vec<CandidateEvaluation>, EvaluationReport> {
    let Some(session) = state.sessions.get(session_id) else {
        return Err(
            EvaluationReportBuilder::new(
                "evaluate_candidates",
                "session",
                Some(session_id.as_str()),
            )
            .error(ErrorEntry::new(
                ReportPhase::StructuralValidation,
                ErrorCode::MissingReference,
                vec![format!("session:{}", session_id.as_str())],
            ))
            .finish(false),
        );
    };

    let effective_votes = derive_effective_votes(&session.votes);
    let mut results = Vec::new();

    for candidate in &session.candidates {
        let mut reasons = Vec::new();
        let mut disposition = CandidateDisposition::Eligible;

        if candidate.round_index != session.round_index {
            disposition = CandidateDisposition::Invalid;
            reasons.push("CANDIDATE_WRONG_ROUND".to_string());
        }

        match &candidate.candidate_payload {
            CandidatePayload::SupersedeResolution {
                supersedes_resolution_id,
                ..
            } => {
                if !state.resolutions.contains_key(supersedes_resolution_id) {
                    disposition = CandidateDisposition::Invalid;
                    reasons.push("MISSING_TARGET_RESOLUTION".to_string());
                }
            }
            CandidatePayload::RetireResolution {
                target_resolution_id,
            } => {
                if !state.resolutions.contains_key(target_resolution_id) {
                    disposition = CandidateDisposition::Invalid;
                    reasons.push("MISSING_TARGET_RESOLUTION".to_string());
                }
            }
            CandidatePayload::AdoptResolution { .. } => {}
        }

        let mut accept_votes = 0;
        let mut reject_votes = 0;

        for participant_votes in effective_votes.values() {
            if participant_votes.effective_accept.as_ref() == Some(&candidate.candidate_id) {
                accept_votes += 1;
            }

            if participant_votes
                .effective_rejects
                .contains(&candidate.candidate_id)
            {
                reject_votes += 1;
            }
        }

        results.push(CandidateEvaluation {
            candidate_id: candidate.candidate_id.clone(),
            disposition,
            reasons,
            accept_votes,
            reject_votes,
        });
    }

    Ok(results)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionStatus {
    NoEligibleCandidates,
    NoAcceptedCandidate,
    UniqueWinner { candidate_id: CandidateId },
    MultipleAcceptedCandidates { candidate_ids: Vec<CandidateId> },
}

pub fn determine_decision_status(
    state: &CompiledState,
    session_id: &SessionId,
) -> Result<DecisionStatus, EvaluationReport> {
    let report = evaluate_session(state, session_id);

    if report.outcome != EvaluationOutcome::Success {
        return Err(report);
    }
    
    let candidates = evaluate_candidates_for_session(state, session_id)?;

    let eligible = candidates
        .iter()
        .filter(|c| c.disposition == CandidateDisposition::Eligible)
        .collect::<Vec<_>>();

    if eligible.is_empty() {
        return Ok(DecisionStatus::NoEligibleCandidates);
    }

    let mut accepted = eligible
        .into_iter()
        .filter(|c| c.accept_votes > 0)
        .map(|c| c.candidate_id.clone())
        .collect::<Vec<_>>();

    accepted.sort();

    match accepted.len() {
        0 => Ok(DecisionStatus::NoAcceptedCandidate),
        1 => Ok(DecisionStatus::UniqueWinner {
            candidate_id: accepted.remove(0),
        }),
        _ => Ok(DecisionStatus::MultipleAcceptedCandidates {
            candidate_ids: accepted,
        }),
    }
}

pub fn can_accept_session(
    state: &CompiledState,
    session_id: &SessionId,
) -> Result<(), EvaluationReport> {
    let report = evaluate_session(state, session_id);

    if report.outcome != EvaluationOutcome::Success {
        return Err(report);
    }

    let decision = determine_decision_status(state, session_id)?;

    match decision {
        DecisionStatus::UniqueWinner { .. } => Ok(()),

        DecisionStatus::NoEligibleCandidates => Err(EvaluationReport::rejected(
            "accept_session",
            "session",
            Some(session_id.as_str()),
            ErrorCode::NoEligibleCandidates,
        )),

        DecisionStatus::NoAcceptedCandidate => Err(EvaluationReport::rejected(
            "accept_session",
            "session",
            Some(session_id.as_str()),
            ErrorCode::AcceptanceConditionsNotMet,
        )),

        DecisionStatus::MultipleAcceptedCandidates { .. } => Err(
            EvaluationReport::rejected(
                "accept_session",
                "session",
                Some(session_id.as_str()),
                ErrorCode::AcceptanceConditionsNotMet,
            ),
        ),
    }
}
