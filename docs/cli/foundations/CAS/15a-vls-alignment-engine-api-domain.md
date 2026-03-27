# Charter Alignment Engine — Data Structures (Rust-Oriented)

Status: FOUNDATIONAL (Implementation-Oriented)  
Applies to: charter_alignment crate (Rust)  
Purpose: Define core structs and traits backing the Alignment Engine API  

---

# 1. Design Goals

- deterministic computation
- minimal allocations where possible
- cache-friendly layout
- separation of compute vs query vs explain
- backend-agnostic
- FFI-friendly evolution

---

# 2. Core Type Aliases

type ResolutionId = String  
type AreaId = String  
type IdentityId = String  

---

# 3. Graph Representation

## 3.1 ResolutionNode

struct ResolutionNode {
    id: ResolutionId,
    parents: Vec<ResolutionId>,
    children: Vec<ResolutionId>,
    area_id: AreaId,
    identity_id: IdentityId,
    is_superseded: bool,
}

---

## 3.2 AlignmentGraph

struct AlignmentGraph {
    nodes: HashMap<ResolutionId, ResolutionNode>,
}

Notes:

- Precomputed adjacency lists are required
- Graph must be acyclic
- Cross-area edges allowed

---

# 4. Signal Model

## 4.1 SemanticState

enum SemanticState {
    Alignment,
    Misalignment,
    Uncertainty,
    ReducedCapacity,
    Pause,
    NeedReassessment,
}

---

## 4.2 ConfidenceLevel

enum ConfidenceLevel {
    Low,
    Medium,
    High,
}

---

## 4.3 SignalRecord

struct SignalRecord {
    resolution_id: ResolutionId,
    state: SemanticState,
    confidence: ConfidenceLevel,
    timestamp: u64,
}

---

# 5. Local Metric Accumulator

## 5.1 LocalStats

struct LocalStats {
    count: u32,
    sum_x: f64,
    sum_x2: f64,
    last_timestamp: u64,
}

Derived at runtime:

- mean
- variance

---

# 6. Derived Resolution State

## 6.1 ResolutionAlignmentState

struct ResolutionAlignmentState {
    resolution_id: ResolutionId,
    mean_drift: f64,
    variance: f64,
    signal_density: f64,
    semantic_state: SemanticState,
    confidence: f64,
    last_signal_time: u64,
    window_size: u64,
}

---

# 7. Area State

struct AreaAlignmentState {
    area_id: AreaId,
    aggregated_drift: f64,
    aggregated_variance: f64,
    capacity_pressure: f64,
    semantic_state: SemanticState,
    confidence: f64,
}

---

# 8. Identity State

struct IdentityAlignmentState {
    identity_id: IdentityId,
    identity_version: u32,
    aggregated_drift: f64,
    transition_volatility: f64,
    semantic_state: SemanticState,
    confidence: f64,
}

---

# 9. Global State

struct GlobalAlignmentState {
    aggregated_drift: f64,
    aggregated_variance: f64,
    active_tension_count: u32,
    confidence: f64,
}

---

# 10. AlignmentStateStore

struct AlignmentStateStore {
    resolution_states: HashMap<ResolutionId, ResolutionAlignmentState>,
    area_states: HashMap<AreaId, AreaAlignmentState>,
    identity_states: HashMap<IdentityId, IdentityAlignmentState>,
    global_state: GlobalAlignmentState,
}

Notes:

- purely derived
- rebuildable
- no raw signals stored here

---

# 11. Update Summary

struct UpdateSummary {
    affected_resolutions: Vec<ResolutionId>,
    affected_areas: Vec<AreaId>,
    affected_identities: Vec<IdentityId>,
    local_updates: u32,
    propagated_updates: u32,
}

---

# 12. Context

struct AlignmentContext {
    current_time: u64,
    window_size: Option<u64>,
}

Future extensions:

- deployment window flags
- experiment flags
- sensitivity tuning

---

# 13. Engine Trait (Core Compute)

trait AlignmentEngine {
    fn rebuild(
        &self,
        graph: &AlignmentGraph,
        signals: &[SignalRecord],
        context: &AlignmentContext,
    ) -> AlignmentStateStore;

    fn apply_signal(
        &self,
        state: &mut AlignmentStateStore,
        graph: &AlignmentGraph,
        signal: &SignalRecord,
        context: &AlignmentContext,
    ) -> UpdateSummary;

    fn apply_structure_change(
        &self,
        state: &mut AlignmentStateStore,
        graph: &AlignmentGraph,
        context: &AlignmentContext,
    ) -> UpdateSummary;

    fn apply_posture_change(
        &self,
        state: &mut AlignmentStateStore,
        context: &AlignmentContext,
    ) -> UpdateSummary;
}

---

# 14. Query Layer (State Access)

impl AlignmentStateStore {

    fn resolution_state(&self, id: &ResolutionId) -> Option<&ResolutionAlignmentState>;

    fn area_state(&self, id: &AreaId) -> Option<&AreaAlignmentState>;

    fn identity_state(&self, id: &IdentityId) -> Option<&IdentityAlignmentState>;

    fn global_state(&self) -> &GlobalAlignmentState;
}

---

# 15. Performance Notes

- prefer preallocated maps where possible
- avoid recomputing adjacency
- reuse LocalStats for incremental updates
- avoid cloning large vectors
- consider arena allocation later if needed

---

# 16. Future Extensions (Planned)

- vector-based resolution representation
- tension graph structures
- time-series storage layer
- adaptive windowing per node
- SIMD optimization for aggregation

---

# 17. Mental Model

Graph = structure  
Signals = observations  
Stats = accumulation  
StateStore = derived snapshot  

Engine transforms signals + graph into state.