# Charter Alignment Engine — API Surface (V1)

Status: FOUNDATIONAL (Implementation-Ready)  
Applies to: charter_alignment library  
Depends On:  
- Alignment Engine — Signal & Local Metrics Specification  
- VLS Reconstructed DAG  
- VDS Signal Model  

Does NOT define: storage backends, UI, dashboards, language bindings  

---

# 1. Purpose

This specification defines the minimal, stable API surface required to:

- compute alignment deterministically
- maintain derived alignment state
- expose read-only alignment queries
- support incremental updates

This API is:

- library-first
- deterministic
- stateless at compute layer
- observational only
- non-authoritative

---

# 2. Core Model

The Alignment Engine is composed of four distinct components:

1. AlignmentEngine (computation only)
2. AlignmentStateStore (derived state + queries)
3. AlignmentExplainer (human-readable reasoning)
4. AlignmentValidator (correctness + consistency checks)

These components must remain logically separate.

---

# 3. Design Principles

## AE-API-01 — Structure + Signals → Derived State

Inputs:
- VLS reconstructed graph
- VDS signals
- optional context

Output:
- derived alignment state

---

## AE-API-02 — Read-Only Query Layer

All queries operate on derived state.

No API may:
- modify legitimacy
- modify identity
- modify signal history

---

## AE-API-03 — Deterministic Rebuild

Given identical inputs:

- graph
- signals
- context

Outputs MUST be identical.

---

## AE-API-04 — Incremental Consistency

Incremental updates must produce the same result as full rebuild.

---

## AE-API-05 — Context Is Interpretive Only

Context MAY affect:
- window sizing
- sensitivity
- tolerance

Context MUST NOT:
- alter signal values
- alter graph structure
- create or modify truth

---

# 4. AlignmentEngine (Compute Layer)

The AlignmentEngine performs computation only.

It does NOT:
- store authoritative data
- expose queries
- interpret meaning

---

## 4.1 Full Rebuild

Recompute all alignment state.

rebuild(graph, signals, context) -> AlignmentStateStore

Used for:
- startup
- replay
- validation

---

## 4.2 Apply Signal Event

Update state when a new signal is recorded.

apply_signal(state_store, graph, signal_event, context) -> UpdateSummary

Effects:
- updates local metrics
- updates propagated metrics

---

## 4.3 Apply Structural Change

Update state when graph topology changes.

Examples:
- supersession
- new references
- identity transitions

apply_structure_change(state_store, graph, structural_event, context) -> UpdateSummary

---

## 4.4 Apply Posture Change

Update interpretive posture.

Examples:
- deployment window
- experiment

apply_posture_change(state_store, graph, posture_event, context) -> UpdateSummary

---

# 5. AlignmentStateStore (Derived State + Queries)

The AlignmentStateStore contains all derived alignment data.

It is:

- rebuildable
- non-authoritative
- query-oriented

Loss of this store must not lose truth.

---

## 5.1 Resolution Queries

resolution_state(resolution_id) -> ResolutionAlignmentState

---

## 5.2 Area Queries

area_state(area_id) -> AreaAlignmentState

---

## 5.3 Identity Queries

identity_state(identity_id) -> IdentityAlignmentState

---

## 5.4 Global Queries

global_alignment_state() -> GlobalAlignmentState

Important:

Global results are scoped to the currently reconstructed graph.  
They do NOT imply complete system truth unless the graph is complete.

---

## 5.5 Structural Queries

### Upstream Influence

upstream_influence(resolution_id) -> InfluenceSet

### Downstream Impact

downstream_impact(resolution_id) -> ImpactSet

### Alignment Cone

alignment_cone(resolution_id) -> AlignmentCone

### Alignment Horizon

alignment_horizon(resolution_id) -> AlignmentHorizonState

---

# 6. Present-State Models

## 6.1 ResolutionAlignmentState

- resolution_id
- mean_drift
- variance
- signal_density
- semantic_state
- confidence
- last_signal_time
- window_size

---

## 6.2 AreaAlignmentState

- area_id
- aggregated_drift
- aggregated_variance
- capacity_pressure
- semantic_state
- confidence

---

## 6.3 IdentityAlignmentState

- identity_id
- identity_version
- aggregated_drift
- transition_volatility
- semantic_state
- confidence

---

## 6.4 GlobalAlignmentState

- aggregated_drift
- aggregated_variance
- active_tension_count
- confidence

---

# 7. UpdateSummary

Returned by incremental updates.

- affected_resolutions
- affected_areas
- affected_identities
- local_updates
- propagated_updates

---

# 8. AlignmentExplainer (Explanation Layer)

Provides human-readable reasoning.

Does not affect computation.

---

## 8.1 Resolution Explanation

explain_resolution(resolution_id) -> Explanation

---

## 8.2 Area Explanation

explain_area(area_id) -> Explanation

---

## 8.3 Identity Explanation

explain_identity(identity_id) -> Explanation

---

# 9. AlignmentValidator (Verification Layer)

Ensures correctness of derived state.

---

## 9.1 Rebuild Validation

validate_rebuild(state_store, graph, signals) -> ValidationReport

---

# 10. Input Contracts

## 10.1 Graph Input

Must include:

- resolution IDs
- parent/child relationships
- supersession edges
- area membership
- identity scope

---

## 10.2 Signal Input

Each signal must include:

- resolution_id
- semantic_state
- confidence
- timestamp

---

# 11. Mental Model

Engine computes  
State store holds derived truth  
Queries read state  
Explainer explains  
Validator verifies  

Nothing modifies legitimacy.

---

# 12. Phased Roadmap (Planned Extensions)

## Phase 2 — Predictive Metrics

- drift_velocity
- drift_acceleration
- phase_transition_risk
- cascade_score

---

## Phase 3 — Advanced Structural Analysis

- tension regions
- attractor detection
- cascade detection

---

## Phase 4 — View Models

- focus view
- trend view
- lineage view
- area health view

---

## Phase 5 — Historical Layer

- resolution_history
- drift timelines
- variance timelines

---

## Phase 6 — Adaptive Windowing

- dynamic window sizing
- variance/density-driven windows

---

## Phase 7 — Federation Awareness

- partial graph confidence
- cross-area completeness awareness

---

# 13. Final Constraint

This API is intentionally minimal.

All complexity must emerge in layers above — never inside this core.
