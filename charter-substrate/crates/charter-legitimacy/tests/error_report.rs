use charter_legitimacy::error::{
    BlockScope, BlockType, ErrorCode, ErrorEntry, EvaluationOutcome, EvaluationReportBuilder,
    ReportPhase,
};

#[test]
fn report_orders_errors_by_phase_code_then_related_objects() {
    let report = EvaluationReportBuilder::new("evaluate_session", "session", Some("s1"))
        .error(ErrorEntry::new(
            ReportPhase::VoteValidation,
            ErrorCode::ParticipantNotFound,
            vec!["participant:p2".into()],
        ))
        .error(ErrorEntry::new(
            ReportPhase::CandidateValidation,
            ErrorCode::MissingReference,
            vec!["candidate:c1".into()],
        ))
        .error(ErrorEntry::new(
            ReportPhase::VoteValidation,
            ErrorCode::ParticipantNotFound,
            vec!["participant:p1".into()],
        ))
        .finish(false);

    let codes = report
        .errors
        .iter()
        .map(|e| (&e.phase, &e.error_code, &e.related_objects))
        .collect::<Vec<_>>();

    assert_eq!(codes[0].0, &ReportPhase::CandidateValidation);
    assert_eq!(codes[0].1, &ErrorCode::MissingReference);

    assert_eq!(codes[1].2, &vec!["participant:p1".to_string()]);
    assert_eq!(codes[2].2, &vec!["participant:p2".to_string()]);
}

#[test]
fn structural_errors_derive_rejected() {
    let report = EvaluationReportBuilder::new("rehydrate_engine", "area_graph", Some("area-1"))
        .error(ErrorEntry::new(
            ReportPhase::StructuralValidation,
            ErrorCode::MissingReference,
            vec!["session:s1".into()],
        ))
        .finish(false);

    assert_eq!(report.outcome, EvaluationOutcome::Rejected);
    assert_eq!(report.primary_error_code, Some(ErrorCode::MissingReference));
}

#[test]
fn blocking_errors_derive_blocked_when_no_rejection_exists() {
    let report = EvaluationReportBuilder::new("evaluate_session", "session", Some("s1"))
        .error(ErrorEntry::blocking(
            ReportPhase::BlockingEvaluation,
            ErrorCode::SessionBlockedTemporary,
            vec!["session:s1".into()],
            BlockType::Temporary,
            BlockScope::Session,
        ))
        .finish(false);

    assert_eq!(report.outcome, EvaluationOutcome::Blocked);
    assert_eq!(
        report.primary_error_code,
        Some(ErrorCode::SessionBlockedTemporary)
    );
}

#[test]
fn success_requires_mutation() {
    let report = EvaluationReportBuilder::new("accept_session", "session", Some("s1"))
        .finish(true);

    assert_eq!(report.outcome, EvaluationOutcome::Success);
    assert!(report.errors.is_empty());
}

#[test]
fn no_errors_and_no_mutation_is_no_op() {
    let report = EvaluationReportBuilder::new("evaluate_session", "session", Some("s1"))
        .finish(false);

    assert_eq!(report.outcome, EvaluationOutcome::NoOp);
    assert!(report.errors.is_empty());
}