use charter_legitimacy::api::engine::{Engine, RehydrateInput};

use charter_legitimacy::domain::{
    AreaGraph, AreaId, Candidate, CandidateId, CandidatePayload, Participant, ParticipantId,
    Session, SessionId, SessionPhase, SessionState, SessionType, Stance, Vote, VoteId, ReversibilityIntent, ResolutionId, 
};

use charter_legitimacy::runtime::{
    CandidateDisposition, DecisionStatus
};

fn make_participant(participant: &str, display_name: &str) -> Participant {
    Participant {
        participant_id: ParticipantId::from(participant),
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        round_index: 1,
        display_name: display_name.into(),
        annotation: None,
        created_at: None,
        schema_version: 1,
    }
}

fn make_candidate(candidate: &str, content: &str) -> Candidate {
    Candidate {
        candidate_id: CandidateId::from(candidate),
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        round_index: 1,
        candidate_payload: CandidatePayload::AdoptResolution {
            resolution_content: content.into(),
        },
        annotation: None,
        reversibility_intent: ReversibilityIntent::Reversible,
        created_at: None,
        schema_version: 1,
    }
}

fn make_accept_vote(participant: &str, candidate: &str) -> Vote {
    Vote {
        vote_id: VoteId::from("v1"),
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        round_index: 1,
        participant_id: ParticipantId::from(participant),
        candidate_id: CandidateId::from(candidate),
        stance: Stance::Accept,
        annotation: None,
        created_at: None,
        schema_version: 1,
    }
}

fn build_engine(session: Session) -> Engine {
    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![session],
        resolutions: vec![],
        receipts: vec![],
    };

    Engine::rehydrate(RehydrateInput { graph })
        .unwrap()
        .engine
        .unwrap()
}

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
            reversibility_intent: ReversibilityIntent::Reversible,
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
fn candidate_is_invalid_if_superseded_target_missing() {
    let mut session = make_session_with_ids();

    session.candidates = vec![Candidate {
        candidate_id: CandidateId::from("c1"),
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        round_index: 1,
        candidate_payload: CandidatePayload::SupersedeResolution {
            resolution_content: "new".into(),
            supersedes_resolution_id: ResolutionId::from("missing"),
        },
        reversibility_intent: ReversibilityIntent::Reversible,
        annotation: None,
        created_at: None,
        schema_version: 1,
    }];

    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![session],
        resolutions: vec![],
        receipts: vec![],
    };

    let engine = Engine::rehydrate(RehydrateInput { graph })
        .unwrap()
        .engine
        .unwrap();

    let results = engine
        .list_session_candidates(SessionId::from("session-1"))
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].disposition, CandidateDisposition::Invalid);
}

#[test]
fn candidate_vote_counts_use_effective_votes() {
    let mut session = make_session_with_ids();

    session.candidates = vec![
        Candidate {
            candidate_id: CandidateId::from("c1"),
            session_id: SessionId::from("session-1"),
            area_id: AreaId::from("area-1"),
            round_index: 1,
            candidate_payload: CandidatePayload::AdoptResolution {
                resolution_content: "test".into(),
            },
            reversibility_intent: ReversibilityIntent::Reversible,
            annotation: None,
            created_at: None,
            schema_version: 1,
        },
        Candidate {
            candidate_id: CandidateId::from("c2"),
            session_id: SessionId::from("session-1"),
            area_id: AreaId::from("area-1"),
            round_index: 1,
            candidate_payload: CandidatePayload::AdoptResolution {
                resolution_content: "other".into(),
            },
            reversibility_intent: ReversibilityIntent::Reversible,
            annotation: None,
            created_at: None,
            schema_version: 1,
        },
    ];

    session.votes = vec![
        Vote {
            vote_id: VoteId::from("v1"),
            session_id: SessionId::from("session-1"),
            area_id: AreaId::from("area-1"),
            round_index: 1,
            participant_id: ParticipantId::from("participant-1"),
            candidate_id: CandidateId::from("c1"),
            stance: Stance::Reject,
            annotation: None,
            created_at: None,
            schema_version: 1,
        },
        Vote {
            vote_id: VoteId::from("v2"),
            session_id: SessionId::from("session-1"),
            area_id: AreaId::from("area-1"),
            round_index: 1,
            participant_id: ParticipantId::from("participant-1"),
            candidate_id: CandidateId::from("c1"),
            stance: Stance::Accept,
            annotation: None,
            created_at: None,
            schema_version: 1,
        },
        Vote {
            vote_id: VoteId::from("v3"),
            session_id: SessionId::from("session-1"),
            area_id: AreaId::from("area-1"),
            round_index: 1,
            participant_id: ParticipantId::from("participant-1"),
            candidate_id: CandidateId::from("c2"),
            stance: Stance::Accept,
            annotation: None,
            created_at: None,
            schema_version: 1,
        },
    ];

    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![session],
        resolutions: vec![],
        receipts: vec![],
    };

    let engine = Engine::rehydrate(RehydrateInput { graph })
        .unwrap()
        .engine
        .unwrap();

    let c1 = engine
        .get_candidate_status(SessionId::from("session-1"), CandidateId::from("c1"))
        .unwrap();

    let c2 = engine
        .get_candidate_status(SessionId::from("session-1"), CandidateId::from("c2"))
        .unwrap();

    assert_eq!(c1.accept_votes, 0);
    assert_eq!(c1.reject_votes, 0);

    assert_eq!(c2.accept_votes, 1);
    assert_eq!(c2.reject_votes, 0);
}

#[test]
fn decision_status_finds_unique_winner() {
    let mut session = make_session_with_ids();

    session.candidates = vec![Candidate {
        candidate_id: CandidateId::from("c1"),
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        round_index: 1,
        candidate_payload: CandidatePayload::AdoptResolution {
            resolution_content: "winner".into(),
        },
        reversibility_intent: ReversibilityIntent::Reversible,
        annotation: None,
        created_at: None,
        schema_version: 1,
    }];

    session.votes = vec![Vote {
        vote_id: VoteId::from("v1"),
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        round_index: 1,
        participant_id: ParticipantId::from("participant-1"),
        candidate_id: CandidateId::from("c1"),
        stance: Stance::Accept,
        annotation: None,
        created_at: None,
        schema_version: 1,
    }];

    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![session],
        resolutions: vec![],
        receipts: vec![],
    };

    let engine = Engine::rehydrate(RehydrateInput { graph })
        .unwrap()
        .engine
        .unwrap();

    let status = engine
        .determine_decision_status(SessionId::from("session-1"))
        .unwrap();

    assert_eq!(
        status,
        DecisionStatus::UniqueWinner {
            candidate_id: CandidateId::from("c1")
        }
    );
}

#[test]
fn decision_status_rejects_multiple_accepted_candidates() {
    let mut session = make_session_with_ids();

    session.candidates = vec![
        Candidate {
            candidate_id: CandidateId::from("c1"),
            session_id: SessionId::from("session-1"),
            area_id: AreaId::from("area-1"),
            round_index: 1,
            candidate_payload: CandidatePayload::AdoptResolution {
                resolution_content: "one".into(),
            },
            reversibility_intent: ReversibilityIntent::Reversible,
            annotation: None,
            created_at: None,
            schema_version: 1,
        },
        Candidate {
            candidate_id: CandidateId::from("c2"),
            session_id: SessionId::from("session-1"),
            area_id: AreaId::from("area-1"),
            round_index: 1,
            candidate_payload: CandidatePayload::AdoptResolution {
                resolution_content: "two".into(),
            },
            reversibility_intent: ReversibilityIntent::Reversible,
            annotation: None,
            created_at: None,
            schema_version: 1,
        },
    ];

    session.participants.push(Participant {
        participant_id: ParticipantId::from("participant-2"),
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        round_index: 1,
        display_name: "Bob".into(),
        annotation: None,
        created_at: None,
        schema_version: 1,
    });

    session.votes = vec![
        Vote {
            vote_id: VoteId::from("v1"),
            session_id: SessionId::from("session-1"),
            area_id: AreaId::from("area-1"),
            round_index: 1,
            participant_id: ParticipantId::from("participant-1"),
            candidate_id: CandidateId::from("c1"),
            stance: Stance::Accept,
            annotation: None,
            created_at: None,
            schema_version: 1,
        },
        Vote {
            vote_id: VoteId::from("v2"),
            session_id: SessionId::from("session-1"),
            area_id: AreaId::from("area-1"),
            round_index: 1,
            participant_id: ParticipantId::from("participant-2"),
            candidate_id: CandidateId::from("c2"),
            stance: Stance::Accept,
            annotation: None,
            created_at: None,
            schema_version: 1,
        },
    ];

    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![session],
        resolutions: vec![],
        receipts: vec![],
    };

    let engine = Engine::rehydrate(RehydrateInput { graph })
        .unwrap()
        .engine
        .unwrap();

    let status = engine
        .determine_decision_status(SessionId::from("session-1"))
        .unwrap();

    assert_eq!(
        status,
        DecisionStatus::MultipleAcceptedCandidates {
            candidate_ids: vec![CandidateId::from("c1"), CandidateId::from("c2")]
        }
    );
}

#[test]
fn session_can_be_accepted_with_unique_winner() {
    let mut session = make_session_with_ids();

    session.candidates = vec![Candidate {
        candidate_id: CandidateId::from("c1"),
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        round_index: 1,
        candidate_payload: CandidatePayload::AdoptResolution {
            resolution_content: "valid".into(),
        },
        reversibility_intent: ReversibilityIntent::Reversible,
        annotation: None,
        created_at: None,
        schema_version: 1,
    }];

    session.votes = vec![Vote {
        vote_id: VoteId::from("v1"),
        session_id: SessionId::from("session-1"),
        area_id: AreaId::from("area-1"),
        round_index: 1,
        participant_id: ParticipantId::from("participant-1"),
        candidate_id: CandidateId::from("c1"),
        stance: Stance::Accept,
        annotation: None,
        created_at: None,
        schema_version: 1,
    }];

    let engine = build_engine(session);

    assert!(engine
        .can_accept_session(SessionId::from("session-1"))
        .is_ok());
}

#[test]
fn session_cannot_be_accepted_with_multiple_winners() {
    let mut session = make_session_with_ids();

    session.participants.push(make_participant("participant-2", "Bob"));

    session.candidates = vec![
        make_candidate("c1", "example"),
        make_candidate("c2", "example2"),
    ];

    session.votes = vec![
        make_accept_vote("participant-1", "c1"),
        make_accept_vote("participant-2", "c2"),
    ];

    let engine = build_engine(session);

    assert!(engine
        .can_accept_session(SessionId::from("session-1"))
        .is_err());
}

