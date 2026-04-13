use charter_legitimacy::compiler::CompiledState;
use charter_legitimacy::domain::{
    AreaGraph, AreaId, CandidateId, CandidatePayload, Receipt,
    ReceiptId, ReceiptType, Resolution, ResolutionId, ResolutionKind, ResolutionState,
    ReversibilityIntent, SessionId, SessionState,
};

#[test]
fn receipt_body_legitimacy_exposes_type_and_resolution_id() {
    let receipt = Receipt {
        receipt_id: ReceiptId::from("r-1"),
        session_id: SessionId::from("s-1"),
        resolution_id: Some(ResolutionId::from("res-1")),
        receipt_type: ReceiptType::Legitimacy,
        resolution_content: Some("example accepted content".into()),
        area_id: AreaId::from("area-1"),
        engine_version: "0.1.0".into(),
        spec_set_hash: "spec-hash".into(),
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

    assert_eq!(receipt.receipt_type, ReceiptType::Legitimacy);
    assert_eq!(
        receipt.resolution_id.as_ref(),
        Some(&ResolutionId::from("res-1"))
    );
}

#[test]
fn receipt_body_exploration_exposes_type_and_no_resolution_id() {
    let receipt = Receipt {
        receipt_id: ReceiptId::from("r-2"),
        session_id: SessionId::from("s-1"),
        resolution_id: None,
        receipt_type: ReceiptType::Exploration,
        resolution_content: None,
        area_id: AreaId::from("area-1"),
        engine_version: "0.1.0".into(),
        spec_set_hash: "spec-hash".into(),
        authority_snapshot_id: None,
        scope_snapshot_id: None,
        problem_statement: None,
        rounds: Vec::new(),
        final_round_index: 1,
        session_state_at_close: SessionState::Closed,
        annotation: None,
        created_at: None,
        hash_algorithm: "sha256".into(),
        content_hash: "abc".into(),
        schema_version: 1,
    };

    assert_eq!(receipt.receipt_type, ReceiptType::Exploration);
    assert_eq!(receipt.resolution_id, None);
}

#[test]
fn candidate_payload_reports_correct_action_type() {
    let adopt = CandidatePayload::AdoptResolution {
        resolution_content: "alpha".into(),
    };

    let supersede = CandidatePayload::SupersedeResolution {
        resolution_content: "beta".into(),
        supersedes_resolution_id: ResolutionId::from("res-1"),
    };

    let retire = CandidatePayload::RetireResolution {
        target_resolution_id: ResolutionId::from("res-9"),
    };

    assert_eq!(
        adopt.action_type(),
        charter_legitimacy::domain::CandidateActionType::AdoptResolution
    );
    assert_eq!(
        supersede.action_type(),
        charter_legitimacy::domain::CandidateActionType::SupersedeResolution
    );
    assert_eq!(
        retire.action_type(),
        charter_legitimacy::domain::CandidateActionType::RetireResolution
    );
}

#[test]
fn compiled_state_builds_deterministic_successor_and_predecessor_indexes() {
    let r1 = Resolution {
        resolution_id: ResolutionId::from("res-1"),
        area_id: AreaId::from("area-1"),
        originating_session_id: SessionId::from("s-1"),
        authority_snapshot_id: None,
        scope_snapshot_id: None,
        accepted_candidate_id: CandidateId::from("c-1"),
        kind: ResolutionKind::Regular,
        engine_version: "0.1.0".into(),
        spec_set_hash: "spec".into(),
        state: ResolutionState::Active,
        superseded_by: Some(ResolutionId::from("res-3")),
        internal_resolution_references: Vec::new(),
        cross_area_references: Vec::new(),
        resolution_content: "example accepted content".into(),
        reversibility_intent: ReversibilityIntent::Reversible,
        annotation: None,
        created_at: None,
        schema_version: 1,
    };

    let r2 = Resolution {
        resolution_id: ResolutionId::from("res-2"),
        area_id: AreaId::from("area-1"),
        originating_session_id: SessionId::from("s-2"),
        authority_snapshot_id: None,
        scope_snapshot_id: None,
        accepted_candidate_id: CandidateId::from("c-2"),
        kind: ResolutionKind::Regular,
        engine_version: "0.1.0".into(),
        spec_set_hash: "spec".into(),
        state: ResolutionState::Active,
        superseded_by: Some(ResolutionId::from("res-3")),
        internal_resolution_references: Vec::new(),
        cross_area_references: Vec::new(),
        resolution_content: "example accepted content".into(),
        reversibility_intent: ReversibilityIntent::Reversible,
        annotation: None,
        created_at: None,
        schema_version: 1,
    };

    let r3 = Resolution {
        resolution_id: ResolutionId::from("res-3"),
        area_id: AreaId::from("area-1"),
        originating_session_id: SessionId::from("s-3"),
        authority_snapshot_id: None,
        scope_snapshot_id: None,
        accepted_candidate_id: CandidateId::from("c-3"),
        kind: ResolutionKind::Regular,
        engine_version: "0.1.0".into(),
        spec_set_hash: "spec".into(),
        state: ResolutionState::Active,
        superseded_by: None,
        internal_resolution_references: Vec::new(),
        cross_area_references: Vec::new(),
        resolution_content: "example accepted content".into(),
        reversibility_intent: ReversibilityIntent::Reversible,
        annotation: None,
        created_at: None,
        schema_version: 1,
    };

    let graph = AreaGraph {
        area_id: Some(AreaId::from("area-1")),
        sessions: Vec::new(),
        resolutions: vec![r2.clone(), r3.clone(), r1.clone()],
        receipts: Vec::new(),
    };

    let compiled = CompiledState::from_graph(graph);

    assert_eq!(
        compiled
            .structural
            .successors_by_resolution
            .get(&ResolutionId::from("res-1"))
            .cloned(),
        Some(vec![ResolutionId::from("res-3")])
    );

    assert_eq!(
        compiled
            .structural
            .successors_by_resolution
            .get(&ResolutionId::from("res-2"))
            .cloned(),
        Some(vec![ResolutionId::from("res-3")])
    );

    assert_eq!(
        compiled
            .structural
            .predecessors_by_resolution
            .get(&ResolutionId::from("res-3"))
            .cloned(),
        Some(vec![ResolutionId::from("res-1"), ResolutionId::from("res-2")])
    );

    assert_eq!(
        compiled.structural.active_resolution_ids,
        vec![ResolutionId::from("res-3")]
    );
} 

#[test]
fn resolution_can_store_reversibility_intent_without_affecting_shape() {
    let resolution = Resolution {
        resolution_id: ResolutionId::from("res-1"),
        area_id: AreaId::from("area-1"),
        originating_session_id: SessionId::from("s-1"),
        authority_snapshot_id: None,
        scope_snapshot_id: None,
        accepted_candidate_id: CandidateId::from("c-1"),
        kind: ResolutionKind::Regular,
        engine_version: "0.1.0".into(),
        spec_set_hash: "spec".into(),
        state: ResolutionState::Active,
        superseded_by: None,
        internal_resolution_references: Vec::new(),
        cross_area_references: Vec::new(),
        resolution_content: "example accepted content".into(),
        reversibility_intent: ReversibilityIntent::ConditionallyReversible,
        annotation: None,
        created_at: None,
        schema_version: 1,
    };

    assert_eq!(
        resolution.reversibility_intent,
        ReversibilityIntent::ConditionallyReversible
    );
}
