use std::collections::{BTreeMap, BTreeSet};

use crate::domain::{
    AreaGraph, Receipt, ReceiptBody, ReceiptId, Resolution, ResolutionId, ResolutionKind, Session,
    SessionId,
};
use crate::error::IntegrityError;

#[derive(Debug, Clone, Default)]
pub struct StructuralIndexes {
    pub successors_by_resolution: BTreeMap<ResolutionId, Vec<ResolutionId>>,
    pub predecessors_by_resolution: BTreeMap<ResolutionId, Vec<ResolutionId>>,
    pub active_resolution_ids: Vec<ResolutionId>,
}

#[derive(Debug, Clone, Default)]
pub struct GovernanceIndexes {
    pub authority_resolution_ids: Vec<ResolutionId>,
    pub scope_resolution_ids: Vec<ResolutionId>,
}

#[derive(Debug, Clone, Default)]
pub struct ReceiptIndexes {
    pub receipts_by_session: BTreeMap<SessionId, Vec<ReceiptId>>,
}

#[derive(Debug, Clone, Default)]
pub struct CompiledState {
    pub area_graph: AreaGraph,
    pub sessions: BTreeMap<SessionId, Session>,
    pub resolutions: BTreeMap<ResolutionId, Resolution>,
    pub receipts: BTreeMap<ReceiptId, Receipt>,
    pub structural: StructuralIndexes,
    pub governance: GovernanceIndexes,
    pub receipt_indexes: ReceiptIndexes,
}

impl CompiledState {
    pub fn from_graph(graph: AreaGraph) -> Self {
        let mut sessions = BTreeMap::new();
        let mut resolutions = BTreeMap::new();
        let mut receipts = BTreeMap::new();

        for session in &graph.sessions {
            sessions.insert(session.session_id.clone(), session.clone());
        }

        for resolution in &graph.resolutions {
            resolutions.insert(resolution.resolution_id.clone(), resolution.clone());
        }

        for receipt in &graph.receipts {
            receipts.insert(receipt.receipt_id.clone(), receipt.clone());
        }

        let mut successors_by_resolution: BTreeMap<ResolutionId, Vec<ResolutionId>> =
            BTreeMap::new();
        let mut predecessors_by_resolution: BTreeMap<ResolutionId, Vec<ResolutionId>> =
            BTreeMap::new();

        for resolution in resolutions.values() {
            if let Some(successor_id) = &resolution.superseded_by {
                successors_by_resolution
                    .entry(resolution.resolution_id.clone())
                    .or_default()
                    .push(successor_id.clone());

                predecessors_by_resolution
                    .entry(successor_id.clone())
                    .or_default()
                    .push(resolution.resolution_id.clone());
            }
        }

        for values in successors_by_resolution.values_mut() {
            values.sort();
            values.dedup();
        }

        for values in predecessors_by_resolution.values_mut() {
            values.sort();
            values.dedup();
        }

        let mut active_resolution_ids = resolutions
            .values()
            .filter(|r| r.state.is_active_structural_candidate() && r.superseded_by.is_none())
            .map(|r| r.resolution_id.clone())
            .collect::<Vec<_>>();
        active_resolution_ids.sort();

        let mut authority_resolution_ids = resolutions
            .values()
            .filter(|r| r.kind == ResolutionKind::Authority)
            .map(|r| r.resolution_id.clone())
            .collect::<Vec<_>>();
        authority_resolution_ids.sort();

        let mut scope_resolution_ids = resolutions
            .values()
            .filter(|r| r.kind == ResolutionKind::Scope)
            .map(|r| r.resolution_id.clone())
            .collect::<Vec<_>>();
        scope_resolution_ids.sort();

        let mut receipts_by_session = BTreeMap::new();
        for receipt in receipts.values() {
            receipts_by_session
                .entry(receipt.session_id.clone())
                .or_insert_with(Vec::new)
                .push(receipt.receipt_id.clone());
        }

        for values in receipts_by_session.values_mut() {
            values.sort();
            values.dedup();
        }

        Self {
            area_graph: graph,
            sessions,
            resolutions,
            receipts,
            structural: StructuralIndexes {
                successors_by_resolution,
                predecessors_by_resolution,
                active_resolution_ids,
            },
            governance: GovernanceIndexes {
                authority_resolution_ids,
                scope_resolution_ids,
            },
            receipt_indexes: ReceiptIndexes { receipts_by_session },
        }
    }
}

pub fn validate_graph(graph: &AreaGraph) -> Vec<IntegrityError> {
    let mut errors = Vec::new();

    validate_single_area(graph, &mut errors);
    validate_duplicate_ids(graph, &mut errors);
    validate_resolution_references(graph, &mut errors);
    validate_session_references(graph, &mut errors);
    validate_receipt_references(graph, &mut errors);

    errors.sort_by(|a, b| {
        let a_code = a.code();
        let b_code = b.code();
        match a_code.cmp(b_code) {
            std::cmp::Ordering::Equal => a.related_objects().cmp(&b.related_objects()),
            other => other,
        }
    });

    errors
}

fn validate_single_area(graph: &AreaGraph, errors: &mut Vec<IntegrityError>) {
    let expected = graph.area_id.as_ref().map(|x| x.as_str());

    for session in &graph.sessions {
        check_area(
            expected,
            session.area_id.as_str(),
            "session",
            session.session_id.as_str(),
            errors,
        );
    }

    for resolution in &graph.resolutions {
        check_area(
            expected,
            resolution.area_id.as_str(),
            "resolution",
            resolution.resolution_id.as_str(),
            errors,
        );
    }

    for receipt in &graph.receipts {
        check_area(
            expected,
            receipt.area_id.as_str(),
            "receipt",
            receipt.receipt_id.as_str(),
            errors,
        );
    }
}

fn check_area(
    expected: Option<&str>,
    found: &str,
    object_type: &str,
    object_id: &str,
    errors: &mut Vec<IntegrityError>,
) {
    if let Some(expected_area) = expected {
        if expected_area != found {
            errors.push(IntegrityError::MultiAreaGraph {
                expected_area_id: expected_area.to_string(),
                found_area_id: found.to_string(),
                object_type: object_type.to_string(),
                object_id: object_id.to_string(),
            });
        }
    }
}

fn validate_duplicate_ids(graph: &AreaGraph, errors: &mut Vec<IntegrityError>) {
    check_duplicates(
        graph.sessions.iter().map(|s| s.session_id.as_str()),
        "session",
        errors,
    );
    check_duplicates(
        graph.resolutions.iter().map(|r| r.resolution_id.as_str()),
        "resolution",
        errors,
    );
    check_duplicates(
        graph.receipts.iter().map(|r| r.receipt_id.as_str()),
        "receipt",
        errors,
    );
}

fn check_duplicates<'a, I>(iter: I, object_type: &str, errors: &mut Vec<IntegrityError>)
where
    I: IntoIterator<Item = &'a str>,
{
    let mut seen = BTreeSet::new();
    let mut duplicates = BTreeSet::new();

    for id in iter {
        if !seen.insert(id.to_string()) {
            duplicates.insert(id.to_string());
        }
    }

    for id in duplicates {
        errors.push(IntegrityError::DuplicateId {
            object_type: object_type.to_string(),
            object_id: id,
        });
    }
}

fn validate_resolution_references(graph: &AreaGraph, errors: &mut Vec<IntegrityError>) {
    let resolution_ids: BTreeSet<&str> = graph
        .resolutions
        .iter()
        .map(|r| r.resolution_id.as_str())
        .collect();

    let session_ids: BTreeSet<&str> = graph.sessions.iter().map(|s| s.session_id.as_str()).collect();

    for resolution in &graph.resolutions {
        if let Some(originating_session_id) = session_ids.get(resolution.originating_session_id.as_str()) {
            let _ = originating_session_id;
        } else {
            errors.push(IntegrityError::MissingReference {
                object_type: "resolution".to_string(),
                object_id: resolution.resolution_id.as_str().to_string(),
                field_name: "originating_session_id".to_string(),
                missing_object_type: "session".to_string(),
                missing_object_id: resolution.originating_session_id.as_str().to_string(),
            });
        }

        if let Some(authority_id) = &resolution.authority_snapshot_id {
            if !resolution_ids.contains(authority_id.as_str()) {
                errors.push(IntegrityError::MissingReference {
                    object_type: "resolution".to_string(),
                    object_id: resolution.resolution_id.as_str().to_string(),
                    field_name: "authority_snapshot_id".to_string(),
                    missing_object_type: "resolution".to_string(),
                    missing_object_id: authority_id.as_str().to_string(),
                });
            }
        }

        if let Some(scope_id) = &resolution.scope_snapshot_id {
            if !resolution_ids.contains(scope_id.as_str()) {
                errors.push(IntegrityError::MissingReference {
                    object_type: "resolution".to_string(),
                    object_id: resolution.resolution_id.as_str().to_string(),
                    field_name: "scope_snapshot_id".to_string(),
                    missing_object_type: "resolution".to_string(),
                    missing_object_id: scope_id.as_str().to_string(),
                });
            }
        }

        if let Some(successor_id) = &resolution.superseded_by {
            if !resolution_ids.contains(successor_id.as_str()) {
                errors.push(IntegrityError::MissingReference {
                    object_type: "resolution".to_string(),
                    object_id: resolution.resolution_id.as_str().to_string(),
                    field_name: "superseded_by".to_string(),
                    missing_object_type: "resolution".to_string(),
                    missing_object_id: successor_id.as_str().to_string(),
                });
            }
        }
    }
}

fn validate_session_references(graph: &AreaGraph, errors: &mut Vec<IntegrityError>) {
    let resolution_ids: BTreeSet<&str> = graph
        .resolutions
        .iter()
        .map(|r| r.resolution_id.as_str())
        .collect();

    let receipt_ids: BTreeSet<&str> = graph.receipts.iter().map(|r| r.receipt_id.as_str()).collect();

    for session in &graph.sessions {
        if let Some(authority_id) = &session.authority_id {
            if !resolution_ids.contains(authority_id.as_str()) {
                errors.push(IntegrityError::MissingReference {
                    object_type: "session".to_string(),
                    object_id: session.session_id.as_str().to_string(),
                    field_name: "authority_id".to_string(),
                    missing_object_type: "resolution".to_string(),
                    missing_object_id: authority_id.as_str().to_string(),
                });
            }
        }

        if let Some(scope_id) = &session.scope_id {
            if !resolution_ids.contains(scope_id.as_str()) {
                errors.push(IntegrityError::MissingReference {
                    object_type: "session".to_string(),
                    object_id: session.session_id.as_str().to_string(),
                    field_name: "scope_id".to_string(),
                    missing_object_type: "resolution".to_string(),
                    missing_object_id: scope_id.as_str().to_string(),
                });
            }
        }

        if let Some(receipt_id) = &session.terminal_receipt_id {
            if !receipt_ids.contains(receipt_id.as_str()) {
                errors.push(IntegrityError::InvalidTerminalReceiptReference {
                    session_id: session.session_id.as_str().to_string(),
                    receipt_id: receipt_id.as_str().to_string(),
                });
            }
        }

        for resolution_ref in &session.internal_resolution_references {
            if !resolution_ids.contains(resolution_ref.as_str()) {
                errors.push(IntegrityError::MissingReference {
                    object_type: "session".to_string(),
                    object_id: session.session_id.as_str().to_string(),
                    field_name: "internal_resolution_references".to_string(),
                    missing_object_type: "resolution".to_string(),
                    missing_object_id: resolution_ref.as_str().to_string(),
                });
            }
        }
    }
}

fn validate_receipt_references(graph: &AreaGraph, errors: &mut Vec<IntegrityError>) {
    let session_ids: BTreeSet<&str> = graph.sessions.iter().map(|s| s.session_id.as_str()).collect();
    let resolution_ids: BTreeSet<&str> = graph
        .resolutions
        .iter()
        .map(|r| r.resolution_id.as_str())
        .collect();

    for receipt in &graph.receipts {
        if !session_ids.contains(receipt.session_id.as_str()) {
            errors.push(IntegrityError::MissingReference {
                object_type: "receipt".to_string(),
                object_id: receipt.receipt_id.as_str().to_string(),
                field_name: "session_id".to_string(),
                missing_object_type: "session".to_string(),
                missing_object_id: receipt.session_id.as_str().to_string(),
            });
        }

        match &receipt.body {
            ReceiptBody::Legitimacy { resolution_id } => {
                if !resolution_ids.contains(resolution_id.as_str()) {
                    errors.push(IntegrityError::InvalidReceiptResolutionBinding {
                        receipt_id: receipt.receipt_id.as_str().to_string(),
                        detail: format!(
                            "LEGITIMACY receipt references missing resolution {}",
                            resolution_id.as_str()
                        ),
                    });
                }
            }
            ReceiptBody::Exploration => {}
        }

        if let Some(authority_id) = &receipt.authority_snapshot_id {
            if !resolution_ids.contains(authority_id.as_str()) {
                errors.push(IntegrityError::MissingReference {
                    object_type: "receipt".to_string(),
                    object_id: receipt.receipt_id.as_str().to_string(),
                    field_name: "authority_snapshot_id".to_string(),
                    missing_object_type: "resolution".to_string(),
                    missing_object_id: authority_id.as_str().to_string(),
                });
            }
        }

        if let Some(scope_id) = &receipt.scope_snapshot_id {
            if !resolution_ids.contains(scope_id.as_str()) {
                errors.push(IntegrityError::MissingReference {
                    object_type: "receipt".to_string(),
                    object_id: receipt.receipt_id.as_str().to_string(),
                    field_name: "scope_snapshot_id".to_string(),
                    missing_object_type: "resolution".to_string(),
                    missing_object_id: scope_id.as_str().to_string(),
                });
            }
        }

        for round in &receipt.rounds {
            for resolution_ref in &round.internal_resolution_references {
                if !resolution_ids.contains(resolution_ref.as_str()) {
                    errors.push(IntegrityError::MissingReference {
                        object_type: "receipt".to_string(),
                        object_id: receipt.receipt_id.as_str().to_string(),
                        field_name: "rounds.internal_resolution_references".to_string(),
                        missing_object_type: "resolution".to_string(),
                        missing_object_id: resolution_ref.as_str().to_string(),
                    });
                }
            }
        }
    }
}