# Charter Alignment Engine — API Surface (V1)

Status: DRAFT (FOUNDATIONAL)  
Applies to: charter_alignment library and runtime-facing consumers  
Depends On: Charter Alignment Engine — End State & Runtime Shape, Signal & Local Metrics Specification  
Does NOT define: storage backend implementation, language bindings, UI behavior, or final wire protocols  

---

# 1. Purpose

This specification defines the intended API surface of the Charter Alignment Engine.

The API exists to support:

- deterministic alignment rebuilds
- incremental alignment updates
- typed read-only alignment queries
- clean integration with VLS and VDS
- runtime facades used by CLI and later bindings

The API is designed to remain:

- library-first
- backend-agnostic
- deterministic
- observational
- non-authoritative

---

# 2. API Design Principles

## AE-API-01 — Structure In, Signals In, Derived State Out
The Alignment Engine consumes:

- reconstructed VLS graph
- VDS signal records
- optional contextual posture metadata

It produces:

- alignment state
- typed derived query results

---

## AE-API-02 — Read-Only Query Surface
The Alignment Engine does not mutate legitimacy, identity, or signal history.

Its public query surface is read-only.

---

## AE-API-03 — Rebuild and Incremental Modes
The engine must support both:

- full rebuild from graph + signals
- incremental update from change events

---

## AE-API-04 — Typed Inputs and Outputs
All public APIs should use typed request and response structures.

Avoid loosely typed maps in core interfaces.

---

## AE-API-05 — Small Composable Queries
The API should prefer small, focused query methods over monolithic summary calls.

---

# 3. Primary Public Components

The public API surface is centered around three concepts:

- AlignmentEngine
- AlignmentStateStore
- typed query result objects

---

# 4. AlignmentEngine

The AlignmentEngine is the computation entry point.

It performs:

- full rebuilds
- incremental updates
- typed state lookups
- higher-order alignment queries

It does not own storage.

---

# 5. Core API Categories

The API surface is divided into four categories:

## 5.1 Build APIs
Used to create or refresh derived alignment state.

## 5.2 Incremental Update APIs
Used when new signals or structural changes occur.

## 5.3 Query APIs
Used to retrieve present and predictive alignment state.

## 5.4 Diagnostic APIs
Used for rebuild validation and explainability support.

---

# 6. Build APIs

## 6.1 Full Rebuild
Purpose:

Recompute all alignment state from:

- reconstructed VLS graph
- signal history
- optional context

Conceptual shape:

rebuild(graph, signals, context) -> AlignmentStateStore

Behavior:

- deterministic
- complete
- rebuildable from scratch

Used for:

- startup
- validation
- replay
- graph topology changes

---

## 6.2 Partial Rebuild
Purpose:

Recompute a scoped subset of alignment state for:

- one area
- one identity
- one subgraph

Conceptual shape:

rebuild_scope(graph, signals, scope, context) -> PartialAlignmentState

Used for:

- large systems
- targeted recalculation
- subsystem analysis

---

# 7. Incremental Update APIs

## 7.1 Apply Signal Event
Purpose:

Update derived alignment state when a new signal/check-in is recorded.

Conceptual shape:

apply_signal(state_store, graph, signal_event, context) -> UpdateSummary

Effects:

- update local metrics
- update semantic state
- update propagated state
- update cached predictive state if enabled

---

## 7.2 Apply Structural Change
Purpose:

Update derived alignment state after topology-relevant changes such as:

- supersession
- new cross-area references
- identity version transitions
- sunset or coexistence changes

Conceptual shape:

apply_structure_change(state_store, graph, structural_event, context) -> UpdateSummary

---

## 7.3 Apply Posture Change
Purpose:

Update interpretive posture when contextual states change, such as:

- deployment window opens
- deployment window closes
- experiment begins
- experiment ends

Conceptual shape:

apply_posture_change(state_store, graph, posture_event, context) -> UpdateSummary

These changes do not alter legitimacy or identity truth.

---

# 8. AlignmentStateStore

The AlignmentStateStore is the derived state produced by the engine.

It is:

- rebuildable
- query-oriented
- non-authoritative
- optionally cacheable

It may contain materialized views for:

- resolution state
- area state
- identity state
- predictive metrics
- tension and influence summaries

Loss of this store must not cause truth loss.

---

# 9. Query APIs

The public query surface should support four structural levels.

## 9.1 Resolution Queries
Used for local honesty and local condition measurement.

Examples:

resolution_state(resolution_id) -> ResolutionAlignmentState  
resolution_history(resolution_id) -> ResolutionAlignmentHistory  
resolution_semantic_path(resolution_id) -> SemanticTransitionPath

---

## 9.2 Area Queries
Used for team/system-level boundaries aligned with Charter Areas.

Examples:

area_state(area_id) -> AreaAlignmentState  
area_pressure(area_id) -> CapacityPressureState  
area_tension(area_id) -> AreaTensionState

---

## 9.3 Identity Queries
Used for department/system-domain/mission-scope analysis from VLS.

Examples:

identity_state(identity_id) -> IdentityAlignmentState  
identity_transition_state(identity_id) -> IdentityTransitionState  
identity_deprecated_regions(identity_id) -> DeprecatedRegionSet

---

## 9.4 Global Queries
Used on the rebuilt federated DAG.

Examples:

global_alignment_state() -> GlobalAlignmentState  
global_tension_regions() -> TensionRegionSet  
global_cascade_candidates() -> CascadeCandidateSet

---

# 10. Present-State Query Results

Present-state objects answer:

“What appears true now?”

These results should expose values such as:

- mean_drift
- variance
- signal_density
- semantic_state
- supportability_state
- confidence
- current_window_size

---

## 10.1 ResolutionAlignmentState
Purpose:

Expose current local alignment condition for one resolution.

Typical fields:

- resolution_id
- mean_drift
- variance
- signal_density
- semantic_state
- supportability_state
- confidence
- last_signal_time
- current_window_size

---

## 10.2 AreaAlignmentState
Purpose:

Expose current alignment condition aggregated at the area level.

Typical fields:

- area_id
- aggregated_drift
- aggregated_variance
- capacity_pressure
- active_tension_count
- semantic_state
- confidence
- last_update_time

---

## 10.3 IdentityAlignmentState
Purpose:

Expose current alignment condition aggregated by VLS identity scope.

Typical fields:

- identity_id
- identity_version
- aggregated_drift
- transition_volatility
- semantic_state
- deprecated_region_count
- coexistence_state
- confidence

---

# 11. Predictive Query Results

Predictive objects answer:

“Where is this moving?”

These results should expose values such as:

- drift_velocity
- drift_acceleration
- cascade_score
- shock_score
- potential
- equilibrium classification
- transition risk

---

## 11.1 ResolutionPredictiveState
Purpose:

Expose predictive local risk or directional change.

Typical fields:

- resolution_id
- drift_velocity
- drift_acceleration
- local_shock_score
- local_cascade_score
- phase_transition_risk

---

## 11.2 AreaPredictiveState
Purpose:

Expose area-level risk and instability patterns.

Typical fields:

- area_id
- pressure_trend
- instability_growth
- cascade_density
- active_shock_count
- equilibrium_classification

---

## 11.3 IdentityPredictiveState
Purpose:

Expose identity-level structural and transition instability.

Typical fields:

- identity_id
- transition_risk
- attractor_shift_score
- deprecation_growth
- coexistence_pressure
- equilibrium_classification

---

# 12. Structural Query Results

Structural results describe how alignment is organized across the DAG.

Examples:

- upstream influence
- downstream impact
- alignment cones
- horizons
- impacted identities
- affected areas

---

## 12.1 Influence Queries
Examples:

upstream_influence(resolution_id) -> InfluenceSet  
downstream_impact(resolution_id) -> ImpactSet

These queries help explain shock and cascade behavior.

---

## 12.2 Cone and Horizon Queries
Examples:

alignment_cone(resolution_id) -> AlignmentCone  
alignment_horizon(resolution_id) -> AlignmentHorizonState

These queries make propagation visible without mutating truth.

---

# 13. Diagnostic APIs

Diagnostic APIs are used for verification and explainability.

Examples:

validate_rebuild(state_store, graph, signals) -> ValidationReport  
explain_resolution_state(resolution_id) -> AlignmentExplanation  
explain_area_state(area_id) -> AlignmentExplanation

These are especially useful for:

- tests
- auditability
- debugging
- AI exegesis layers

---

# 14. UpdateSummary

Incremental APIs should return a typed update summary.

Purpose:

Describe what changed without requiring the caller to diff the full store.

Typical fields:

- affected_resolutions
- affected_areas
- affected_identities
- local_metrics_updated
- propagated_metrics_updated
- predictive_metrics_updated
- semantic_state_changes

This allows runtimes to refresh only what changed.

---

# 15. Context Object

The engine may consume an AlignmentContext object.

Purpose:

Provide non-authoritative interpretive context for computation.

Examples of context:

- active deployment windows
- active experiments
- posture overrides
- current clock/time source
- threshold interpretation mode

The context object may affect:

- window sizing
- sensitivity
- tolerance posture
- aggregation cadence

It may never alter legitimacy or identity truth.

---

# 16. Expected Input Contracts

## 16.1 Graph Input
The engine expects a reconstructed VLS graph that includes:

- node identifiers
- typed edges
- area membership
- identity scope metadata
- supersession relationships
- cross-area references where available

## 16.2 Signal Input
The engine expects normalized signal artifacts or source inputs convertible into normalized signals.

Each signal must include at minimum:

- target resolution
- semantic state
- confidence level
- timestamp

---

# 17. Runtime Facade Expectations

Although the Alignment Engine is library-first, a runtime/facade layer may expose simpler orchestration methods.

Examples:

- rebuild_alignment()
- update_alignment_from_signal()
- query_resolution_alignment()
- query_area_alignment()
- query_identity_alignment()

The runtime layer coordinates stores and loading.
The engine remains computation-only.

---

# 18. Boundaries

The Alignment Engine API may:

- compute
- aggregate
- classify
- explain derived state

The Alignment Engine API may never:

- create legitimacy
- mutate decisions
- mutate identity
- record authoritative truth by itself
- infer authority
- enforce action

---

# 19. Mental Model

Build APIs create derived state.  
Incremental APIs keep derived state current.  
Query APIs expose present, predictive, and structural views.  
Diagnostic APIs explain why the engine reached its conclusions.

The API is shaped around visibility, not control.

---

# 20. Why This Matters

A stable API surface allows:

- CLI integration
- Python and .NET bindings later
- deterministic tests
- replay and simulation
- software observability use cases
- team-level and organization-level introspection

This API is the contract that turns the Alignment Engine from a theory into a reusable platform component.