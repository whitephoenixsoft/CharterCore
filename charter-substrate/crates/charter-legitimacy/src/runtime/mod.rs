use std::collections::{BTreeMap, BTreeSet};

use crate::compiler::CompiledState;
use crate::domain::{
    CandidateId, CandidatePayload, ParticipantId, SessionId, SessionPhase, SessionState, Stance,
    Vote,
};
use crate::error::{ErrorEntry, EvaluationOutcome, EvaluationReport};

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
        return EvaluationReport::rejected(
            "evaluate_session",
            "session",
            Some(session_id.as_str()),
            "SESSION_NOT_FOUND",
        );
    };

    let mut errors = Vec::new();

    if matches!(session.state, SessionState::Accepted | SessionState::Closed) {
        errors.push(ErrorEntry {
            error_code: "SESSION_TERMINAL_IMMUTABLE".to_string(),
            related_objects: vec![format!("session:{}", session.session_id.as_str())],
            block_type: None,
            block_scope: None,
        });
    }

    if session.phase == SessionPhase::PreStance && !session.votes.is_empty() {
        errors.push(ErrorEntry {
            error_code: "INVALID_STATE_COMBINATION".to_string(),
            related_objects: vec![format!("session:{}", session.session_id.as_str())],
            block_type: None,
            block_scope: None,
        });
    }

    if session.candidates.is_empty() {
        errors.push(ErrorEntry {
            error_code: "NO_ELIGIBLE_CANDIDATES".to_string(),
            related_objects: vec![format!("session:{}", session.session_id.as_str())],
            block_type: None,
            block_scope: None,
        });
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
            errors.push(ErrorEntry {
                error_code: "INVALID_PARTICIPANT_EPOCH".to_string(),
                related_objects: vec![format!("candidate:{}", candidate.candidate_id.as_str())],
                block_type: None,
                block_scope: None,
            });
        }
    }

    for vote in &session.votes {
        if vote.round_index != session.round_index {
            errors.push(ErrorEntry {
                error_code: "INVALID_PARTICIPANT_EPOCH".to_string(),
                related_objects: vec![format!("vote:{}", vote.vote_id.as_str())],
                block_type: None,
                block_scope: None,
            });
        }

        if !participant_ids.contains(vote.participant_id.as_str()) {
            errors.push(ErrorEntry {
                error_code: "PARTICIPANT_NOT_FOUND".to_string(),
                related_objects: vec![
                    format!("vote:{}", vote.vote_id.as_str()),
                    format!("participant:{}", vote.participant_id.as_str()),
                ],
                block_type: None,
                block_scope: None,
            });
        }

        if !candidate_ids.contains(vote.candidate_id.as_str()) {
            errors.push(ErrorEntry {
                error_code: "MISSING_REFERENCE".to_string(),
                related_objects: vec![
                    format!("vote:{}", vote.vote_id.as_str()),
                    format!("candidate:{}", vote.candidate_id.as_str()),
                ],
                block_type: None,
                block_scope: None,
            });
        }
    }

    errors.sort_by(|a, b| match a.error_code.cmp(&b.error_code) {
        std::cmp::Ordering::Equal => a.related_objects.cmp(&b.related_objects),
        other => other,
    });

    if errors.is_empty() {
        EvaluationReport::success("evaluate_session", "session", Some(session_id.as_str()))
    } else {
        EvaluationReport {
            evaluation_id: None,
            command_type: "evaluate_session".to_string(),
            target_object_type: "session".to_string(),
            target_object_id: Some(session_id.as_str().to_string()),
            outcome: EvaluationOutcome::Rejected,
            primary_error_code: errors.first().map(|e| e.error_code.clone()),
            errors,
            diagnostics: None,
            occurred_at: None,
            schema_version: 1,
        }
    }
}

pub fn evaluate_candidates_for_session(
    state: &CompiledState,
    session_id: &SessionId,
) -> Result<Vec<CandidateEvaluation>, EvaluationReport> {
    let Some(session) = state.sessions.get(session_id) else {
        return Err(EvaluationReport::rejected(
            "evaluate_candidates",
            "session",
            Some(session_id.as_str()),
            "SESSION_NOT_FOUND",
        ));
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