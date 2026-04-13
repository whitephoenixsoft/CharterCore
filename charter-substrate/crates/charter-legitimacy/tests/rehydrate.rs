use charter_legitimacy::api::engine::{Engine, RehydrateInput, RuntimeMode};
use charter_legitimacy::domain::{
    AreaGraph, AreaId, CandidateId, Receipt, ReceiptId, Resolution,
    ResolutionId, ResolutionKind, ResolutionState, ReversibilityIntent, Session, SessionId,
    SessionPhase, SessionState, SessionType, ReceiptType
};

fn make_session(id: &str, area: &str) -> Session {
    Session {
        session_id: SessionId::from(id),
        area_id: AreaId::from(area),
        session_type: SessionType::Regular,
        authority_id: None,
        scope_id: None,
        phase: SessionPhase::PreStance,
        state: SessionState::Active,
        round_index: 1,
        participants: Vec::new(),
        candidates: Vec::new(),
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

fn make_resolution(id: &str, area: &str, session_id: &str) -> Resolution {
    Resolution {
        resolution_id: ResolutionId::from(id),
        area_id: AreaId::from(area),
        originating_session_id: SessionId::from(session_id),
        authority_snapshot_id: None,
        scope_snapshot_id: None,
        accepted_candidate_id: CandidateId::from("candidate-1"),
        resolution_content: "some resolution".into(),
        kind: ResolutionKind::Regular,
        engine_version: "0.1.0".into(),
        spec_set_hash: "spec".into(),
        state: ResolutionState::Active,
        superseded_by: None,
        internal_resolution_references: Vec::new(),
        cross_area_references: Vec::new(),
        reversibility_intent: ReversibilityIntent::Reversible,
        annotation: None,
        created_at: None,
        schema_version: 1,
    }
}

#[test]
fn rehydrate_succeeds_for_valid_single_area_graph() {
    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![make_session("session-1", "area-1")],
        resolutions: vec![make_resolution("resolution-1", "area-1", "session-1")],
        receipts: Vec::new(),
    };

    let result = Engine::rehydrate(RehydrateInput { graph }).expect("rehydrate should not error");

    assert_eq!(result.runtime_mode, Some(RuntimeMode::NormalRuntime));
    assert!(result.engine.is_some());
    assert!(result.report.errors.is_empty());
}

#[test]
fn rehydrate_rejects_multi_area_graph() {
    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![
            make_session("session-1", "area-1"),
            make_session("session-2", "area-2"),
        ],
        resolutions: Vec::new(),
        receipts: Vec::new(),
    };

    let result = Engine::rehydrate(RehydrateInput { graph }).expect("rehydrate should return report");

    assert!(result.engine.is_none());
    assert_eq!(result.runtime_mode, None);
    assert_eq!(
        result.report.primary_error_code.as_deref(),
        Some("MULTI_AREA_GRAPH_DETECTED")
    );
}

#[test]
fn rehydrate_rejects_missing_resolution_reference_from_session() {
    let mut session = make_session("session-1", "area-1");
    session.authority_id = Some(ResolutionId::from("missing-authority"));

    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![session],
        resolutions: Vec::new(),
        receipts: Vec::new(),
    };

    let result = Engine::rehydrate(RehydrateInput { graph }).expect("rehydrate should return report");

    assert!(result.engine.is_none());
    assert_eq!(
        result.report.primary_error_code.as_deref(),
        Some("MISSING_REFERENCE")
    );
}

#[test]
fn rehydrate_rejects_legitimacy_receipt_with_missing_resolution() {
    let session = make_session("session-1", "area-1");

    let receipt = Receipt {
        receipt_id: ReceiptId::from("receipt-1"),
        session_id: SessionId::from("session-1"),
        resolution_id: Some(ResolutionId::from("missing-resolution")),
        receipt_type: ReceiptType::Legitimacy,
        resolution_content: Some("example accepted content".into()),
        area_id: AreaId::from("area-1"),
        engine_version: "0.1.0".into(),
        spec_set_hash: "spec".into(),
        authority_snapshot_id: None,
        scope_snapshot_id: None,
        problem_statement: None,
        rounds: Vec::new(),
        final_round_index: 1,
        session_state_at_close: SessionState::Accepted,
        annotation: None,
        created_at: None,
        hash_algorithm: "sha256".into(),
        content_hash: "abc".into(),
        schema_version: 1,
    };

    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: vec![session],
        resolutions: Vec::new(),
        receipts: vec![receipt],
    };

    let result = Engine::rehydrate(RehydrateInput { graph }).expect("rehydrate should return report");

    assert!(result.engine.is_none());
    assert_eq!(
        result.report.primary_error_code.as_deref(),
        Some("INVALID_RECEIPT_RESOLUTION_BINDING")
    );
}
