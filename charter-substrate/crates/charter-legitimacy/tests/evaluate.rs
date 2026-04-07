use charter_legitimacy::api::engine::{Engine, RehydrateInput};
use charter_legitimacy::domain::{
    AreaGraph, AreaId, Candidate, CandidateId, CandidatePayload, Participant, ParticipantId,
    Session, SessionId, SessionPhase, SessionState, SessionType, Stance, Vote, VoteId,
};
use charter_legitimacy::error::EvaluationOutcome;

fn make_session_with_ids() -> Session {
    Session {
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        session_type: SessionType::Regular,
        authority_id: None,
        scope_id: None,
        phase: SessionPhase::Voting,
        state: SessionState::Active,
        round_index: 1,
        participants: vec![Participant {
            participant_id: ParticipantId::from("participant-1"),
            session_id: SessionId::from("session-1"),
            area_id: AreaId::from("area-1"),
            round_index: 1,
            display_name: "Alice".into(),
            annotation: None,
            created_at: None,
            schema_version: 1,
        }],
        candidates: vec![Candidate {
            candidate_id: CandidateId::from("candidate-1"),
            session_id: SessionId::from("session-1"),
            area_id: AreaId::from("area-1"),
            round_index: 1,
            candidate_payload: CandidatePayload::AdoptResolution {
                resolution_content: "example".into(),
            },
            annotation: None,
            created_at: None,
            schema_version: 1,
        }],
        constraints: Vec::new(),
        votes: Vec::new(),
        internal_resolution_references: Vec::new(),
        cross_area_references: Vec::new(),
        terminal_receipt_id: None,
        annotation: None,
        created_at: None,
        updated_at: None,
        schema_version: 1,
    }
}

#[test]
fn evaluate_session_succeeds_for_basic_valid_session() {
    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![make_session_with_ids()],
        resolutions: Vec::new(),
        receipts: Vec::new(),
    };

    let result = Engine::rehydrate(RehydrateInput { graph }).unwrap();
    let engine = result.engine.expect("engine should exist");

    let report = engine.evaluate_session(SessionId::from("session-1")).unwrap();

    assert_eq!(report.outcome, EvaluationOutcome::Success);
    assert!(report.errors.is_empty());
}

#[test]
fn evaluate_session_rejects_empty_candidate_set() {
    let mut session = make_session_with_ids();
    session.candidates.clear();

    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![session],
        resolutions: Vec::new(),
        receipts: Vec::new(),
    };

    let result = Engine::rehydrate(RehydrateInput { graph }).unwrap();
    let engine = result.engine.expect("engine should exist");

    let report = engine.evaluate_session(SessionId::from("session-1")).unwrap();

    assert_eq!(report.outcome, EvaluationOutcome::Rejected);
    assert_eq!(report.primary_error_code.as_deref(), Some("NO_ELIGIBLE_CANDIDATES"));
}

#[test]
fn evaluate_session_rejects_vote_with_missing_participant() {
    let mut session = make_session_with_ids();
    session.votes.push(Vote {
        vote_id: VoteId::from("vote-1"),
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        round_index: 1,
        participant_id: ParticipantId::from("missing-participant"),
        candidate_id: CandidateId::from("candidate-1"),
        stance: Stance::Accept,
        annotation: None,
        created_at: None,
        schema_version: 1,
    });

    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![session],
        resolutions: Vec::new(),
        receipts: Vec::new(),
    };

    let result = Engine::rehydrate(RehydrateInput { graph }).unwrap();
    let engine = result.engine.expect("engine should exist");

    let report = engine.evaluate_session(SessionId::from("session-1")).unwrap();

    assert_eq!(report.outcome, EvaluationOutcome::Rejected);
    assert_eq!(report.primary_error_code.as_deref(), Some("PARTICIPANT_NOT_FOUND"));
}

#[test]
fn evaluate_session_rejects_vote_with_missing_candidate() {
    let mut session = make_session_with_ids();
    session.votes.push(Vote {
        vote_id: VoteId::from("vote-1"),
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        round_index: 1,
        participant_id: ParticipantId::from("participant-1"),
        candidate_id: CandidateId::from("missing-candidate"),
        stance: Stance::Accept,
        annotation: None,
        created_at: None,
        schema_version: 1,
    });

    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![session],
        resolutions: Vec::new(),
        receipts: Vec::new(),
    };

    let result = Engine::rehydrate(RehydrateInput { graph }).unwrap();
    let engine = result.engine.expect("engine should exist");

    let report = engine.evaluate_session(SessionId::from("session-1")).unwrap();

    assert_eq!(report.outcome, EvaluationOutcome::Rejected);
    assert_eq!(report.primary_error_code.as_deref(), Some("MISSING_REFERENCE"));
}