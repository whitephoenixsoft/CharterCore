use std::collections::BTreeMap;

use crate::domain::{
    AreaGraph, Receipt, ReceiptId, Resolution, ResolutionId, Session, SessionId,
};

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

        let active_resolution_ids = resolutions
            .values()
            .filter(|r| r.state.is_active_structural_candidate() && r.superseded_by.is_none())
            .map(|r| r.resolution_id.clone())
            .collect::<Vec<_>>();

        let authority_resolution_ids = resolutions
            .values()
            .filter(|r| r.kind == crate::domain::ResolutionKind::Authority)
            .map(|r| r.resolution_id.clone())
            .collect::<Vec<_>>();

        let scope_resolution_ids = resolutions
            .values()
            .filter(|r| r.kind == crate::domain::ResolutionKind::Scope)
            .map(|r| r.resolution_id.clone())
            .collect::<Vec<_>>();

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