# Charter Alignment Engine — End State & Runtime Shape

Status: DRAFT (FOUNDATIONAL)  
Applies to: Charter Alignment Engine, VLS, VDS, Automated VDS/VLS Systems  
Depends On: Charter Legitimacy Engine, VLS DAG Reconstruction, VDS Signal Model  
Does NOT define: Signal encoding formulas, graph propagation formulas, storage backend implementation, or UI presentation  

---

# 1. Purpose

This specification defines the intended end state and runtime shape of the Charter Alignment Engine.

The Alignment Engine exists to compute the relationship between:

- explicit decisions
- observed conditions
- structural identity and scope

It does this without:

- creating legitimacy
- modifying identity
- enforcing action
- inferring authority

The Alignment Engine is a computational layer.

It is observational, deterministic, and derived.

---

# 2. Position in the Charter Platform

The Charter platform consists of several distinct modules.

## 2.1 Charter Legitimacy
Charter legitimacy records explicit decisions, supersession, and legitimacy history.

It provides:

- resolutions
- sessions
- areas
- supersession lineage
- authority and scope validation

It does not interpret behavior.

---

## 2.2 Charter Orchestrator
The orchestrator adds pre-legitimacy work such as:

- deliberates
- breakouts
- options
- synthesis
- baseline review

It prepares proposals for legitimacy.

It does not compute alignment.

---

## 2.3 VDS
VDS provides caregiving observations relative to decisions.

It emits:

- check-ins
- requests
- supportability observations
- deployment/experiment posture context

It does not compute global alignment dynamics.

---

## 2.4 VLS
VLS provides structural identity and lineage.

It defines:

- identities
- identity versions
- scope boundaries
- sunset and deprecation
- reconstructed distributed DAGs

It does not interpret runtime behavior.

---

## 2.5 Alignment Engine
The Alignment Engine consumes outputs from Charter, VDS, and VLS and computes alignment dynamics.

It is the first place where the relationship between:

- intent
- behavior
- identity

is mechanically evaluated together.

---

# 3. Core Function

The Alignment Engine answers questions such as:

- How is this resolution behaving relative to its declared intent?
- Is this instability local or systemic?
- Which areas or identities are under pressure?
- Is alignment improving or degrading?
- What changed after a supersession event?
- Are local decisions forming an upward cascade?
- Are higher-level changes creating downstream shock?

The Alignment Engine computes these answers.
It never converts them into obligation.

---

# 4. Primary Inputs

The Alignment Engine has two primary inputs and several optional contextual inputs.

## 4.1 Primary Structural Input
The primary structural input is the VLS reconstructed DAG.

This DAG may include:

- local area graphs
- federated area graphs
- rebuilt global DAGs
- identity-scoped subgraphs

The Alignment Engine does not construct legitimacy.
It consumes a DAG already reconstructed by VLS.

---

## 4.2 Primary Observational Input
The primary observational input is the signal stream.

Signals may come from:

- human VDS check-ins
- automated VDS metric interpretation
- deployment posture context
- experiment context

Signals are observational only.

---

## 4.3 Optional Contextual Inputs
The engine may also consume:

- identity metadata
- area metadata
- deployment windows
- experiment windows
- deprecation and sunset states
- metric thresholds associated with resolutions

These modify interpretation, not legitimacy.

---

# 5. Runtime Modes

The Alignment Engine supports two runtime modes.

## 5.1 Full Rebuild
Used when:

- loading from commit history
- rebuilding from zero
- validating derived state
- rehydrating after restart

Flow:

- load reconstructed DAG
- load signal history
- compute local metrics
- compute semantic states
- propagate influence
- compute higher-order dynamics
- materialize derived state

---

## 5.2 Incremental Update
Used when:

- new signals arrive
- a new resolution supersedes another
- a deployment window opens or closes
- VLS topology changes
- identity version changes

Flow:

- update affected node state
- update dependent aggregates
- update propagated states
- update materialized derived state

The engine must support deterministic replay in both modes.

---

# 6. Internal Runtime Layers

The Alignment Engine operates in layers.

## 6.1 Local Metrics Layer
Computes per-resolution scalar metrics such as:

- mean drift
- variance
- signal density
- recency
- confidence-weighted values

This is the lowest computational layer.

---

## 6.2 Semantic State Layer
Applies the semantic lattice to local metrics and signal transitions.

This produces:

- semantic state
- state transition labels
- present descriptive posture

This layer interprets signals without assigning blame.

---

## 6.3 Propagation Layer
Uses DAG structure to propagate and aggregate effects through:

- supersession edges
- resolution references
- area references
- identity scopes

This layer computes:

- influence decay
- alignment cones
- alignment horizons
- ancestor and descendant effects

---

## 6.4 Dynamics Layer
Computes higher-order alignment dynamics such as:

- goal influence
- tension indicators
- cascades
- shock propagation
- potential
- equilibrium tendency

This layer expresses the field-like behavior of the system.

---

## 6.5 Materialization Layer
Stores derived state for efficient query and dashboard use.

This is the Alignment State Store.

It is derived, rebuildable, and non-authoritative.

---

# 7. End-State Outputs

The Alignment Engine produces three classes of output.

## 7.1 Present-State Outputs
These describe what appears true now.

Examples:

- mean drift
- variance
- signal density
- semantic state
- capacity posture
- supportability posture
- active tension indicators
- local confidence

---

## 7.2 Predictive Outputs
These describe where the system appears to be moving.

Examples:

- drift velocity
- drift acceleration
- instability growth
- cascade emergence
- shock intensity
- attractor strengthening
- transition risk

---

## 7.3 Structural Outputs
These describe how alignment is organized.

Examples:

- alignment cones
- horizons
- identity-scoped regions
- upstream/downstream influence
- area-level aggregation
- global alignment partitions

---

# 8. Alignment State Store

The Alignment State Store is the materialized derived state store used by the Alignment Engine.

It replaces the earlier idea of a drift store.

## 8.1 Purpose
The Alignment State Store exists to:

- accelerate read/query paths
- preserve derived time-series state
- support dashboards and analysis
- avoid full recomputation on every query

It is never the source of truth.

---

## 8.2 Properties
The Alignment State Store is:

- derived
- rebuildable
- query-oriented
- append-aware
- non-authoritative

Loss of the store must be non-fatal.

---

## 8.3 Typical Stored Views
The store may materialize:

### Per Resolution
- mean_drift
- variance
- signal_density
- semantic_state
- last_signal_time
- active_window_size
- supportability_state

### Per Area
- aggregated_drift
- capacity_pressure
- area_variance
- signal_density
- tension_count

### Per Identity
- identity influence
- scope stability
- transition volatility
- deprecation-related alignment regions

### Predictive State
- drift_velocity
- drift_acceleration
- cascade_score
- shock_score
- potential
- equilibrium classification

---

# 9. Present vs Predictive Distinction

The Alignment Engine must maintain a hard conceptual separation between present and predictive outputs.

## 9.1 Present
Present metrics answer:

“What appears true now?”

Examples:

- current drift
- current variance
- current semantic state
- current supportability posture

---

## 9.2 Predictive
Predictive metrics answer:

“Where is this likely moving?”

Examples:

- drift velocity
- drift acceleration
- shock propagation
- cascade emergence
- phase transition risk

Predictive metrics are observational projections, not commitments.

---

# 10. Semantic vs Numeric Distinction

The engine uses a hybrid model.

## 10.1 Numeric Layer
Used for:

- arithmetic
- rolling metrics
- variance
- propagation
- weighting
- thresholds

---

## 10.2 Semantic Layer
Used for:

- semantic lattice
- named transition paths
- operator-facing states
- explanation and exegesis

Neither layer replaces the other.

The numeric layer computes.
The semantic layer explains.

---

# 11. Identity and Area Scope

The engine must support alignment analysis across multiple structural levels.

## 11.1 Resolution Level
Used for local honesty and local condition measurement.

## 11.2 Area Level
Used for local team/system boundaries and legitimacy partitions.

## 11.3 Identity Level
Used for departments, system domains, or structural responsibility groups.

## 11.4 Global Level
Used after VLS reconstructs a federated DAG.

The same engine must support all four levels without changing legitimacy semantics.

---

# 12. Supersession Semantics

Supersession affects the alignment engine in two different ways.

## 12.1 Present State
Only the active resolution is used for current-state alignment.

## 12.2 Historical Dynamics
Superseded resolutions remain part of lineage history and continue to affect:

- momentum
- shock propagation
- gravity
- attractor shifts
- phase transition detection

Supersession must be treated as a first-class dynamic event.

---

# 13. Deployment and Transition Context

The engine must understand declared transition context.

Examples:

- deployment windows
- experiments
- identity version transitions
- sunset periods
- coexistence periods

These contexts do not change legitimacy or identity truth.

They adjust:

- interpretive posture
- tolerance bands
- monitoring sensitivity
- aggregation cadence

---

# 14. Metric-Linked Resolutions

Some resolutions may define expected observables through associated metrics and thresholds.

Examples:

- latency thresholds
- uptime expectations
- capacity tolerances
- business invariants

When such expected observables are present, the Alignment Engine may derive automated signals by comparing:

- declared thresholds
- observed telemetry

This is how automated VDS integrates with the Charter platform.

---

# 15. Boundary Rules

The Alignment Engine may:

- compute
- aggregate
- propagate
- classify
- materialize derived state

The Alignment Engine may never:

- create legitimacy
- change decisions
- mutate identity
- enforce action
- infer authority
- suppress disagreement
- convert visibility into obligation

---

# 16. Design Guarantees

The Alignment Engine MUST remain:

- deterministic
- observational
- non-authoritative
- rebuildable
- backend-agnostic

All outputs are derived from upstream truth and context.

---

# 17. Mental Model

Charter defines what was explicitly decided.  
VLS defines who the structure claims to be.  
VDS defines what is being observed.  
The Alignment Engine computes how those realities relate.

The Alignment State Store remembers the computed condition of that relationship.

---

# 18. Why This Matters

Without the Alignment Engine:

- decisions remain isolated from behavior
- identity remains isolated from intent
- observability remains disconnected from declared purpose

With it:

- teams can see themselves honestly
- organizations can observe structural drift
- software systems can be monitored relative to declared intent
- transitions can be measured without coercion

This is the first layer where intent, identity, and reality become mechanically legible together.