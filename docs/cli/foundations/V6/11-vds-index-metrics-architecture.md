# RFC: Charter Engine Index & Metrics Architecture (V1–V7 Context)

## Status
Draft / Conceptual — For review and alignment before implementation

---

## Purpose
Provide a standard, backend-agnostic architecture for:
- Fast query of resolution, goal, and area metrics
- Alignment and drift analysis
- Tension vector and semantic variance tracking
- Incremental updates from commit and signal streams

Supports V4+ design goals:
- Abstract persistence layer
- Multiple storage backends (SQL, KV, in-memory)
- Incremental compilation of DAG
- Analytical computation for alignment and governance observability

---

## Principles
1. **Indexes are derived** — No index is authoritative.
2. **Resolutions remain the smallest legitimacy unit** — All metrics and views are derived.
3. **Engine is stateless and memory-driven** — Rebuild indexes on rehydration.
4. **Incremental updates** — Only recompute affected nodes and ancestor metrics on new commits or signals.
5. **Backend agnostic** — Indexes operate over trait-based stores.
6. **Append-only history** — Commit and signal stores are immutable; metrics derive, never mutate, historical events.

---

## Index Definitions

### 1. Resolution Index
**Key:** resolution_id  
**Value:**
- parent_ids (Vec<ResolutionID>)
- child_ids (Vec<ResolutionID>)
- area_id
- status (active, superseded, retired)
- local_metrics:
  - drift_score
  - variance
  - signal_counts
  - latest_commit_id

**Purpose:**  
Direct lookup of resolution metrics, graph traversal for goal/area aggregation.

---

### 2. Goal Influence Index
**Key:** goal_id (resolution acting as goal)  
**Value:**
- descendant_resolution_ids (Set<ResolutionID>)
- aggregated_metrics:
  - influence_score
  - descendant_drift
  - descendant_variance
- last_updated_timestamp

**Purpose:**  
Support higher-level aggregation, alignment cones, and influence weighting.

---

### 3. Area Metrics Index
**Key:** area_id  
**Value:**
- resolution_ids (Set<ResolutionID>)
- capacity_signals (count per type)
- pressure_index (normalized)
- signal_variance
- last_updated_timestamp

**Purpose:**  
Operational context for dashboards, alignment health, and capacity monitoring.

---

### 4. Optional Derived Indexes
- **Tension Vector Index**
  - Tracks conflicts along the resolution DAG
  - Weighted by drift, semantic variance, intent gravity
- **Time Window Index**
  - Aggregates metrics per resolution over user-defined windows
  - Supports alignment horizon, trending, and monitoring

---

## Incremental Update Flow
1. New commit/signal arrives
2. Update resolution_index for target node
3. Update goal_influence_index for affected ancestors
4. Update area_metrics_index for associated area
5. Optionally update tension vector and time window indexes

- Full index rebuild occurs only during engine rehydration

---

## Backend Agnosticism
- Index traits support:
  - Relational DB (Postgres/SQLite)
  - Key-value stores (RocksDB, Redis)
  - In-memory structures
- Persistence optional; indexes are derived from commits and signals

---

## Metric Computation & Alignment Concepts
- **Drift Score:** Measures variance or semantic divergence
- **Intent Gravity:** Weight of signals in influencing higher-level resolutions
- **Tension Vector:** Aggregated conflicts in DAG paths
- **Alignment Cones / Horizons:** Visualized uncertainty propagation to ancestors
- **Capacity Pressure Monitor:** Normalize signal density over time windows
- **Signal Types:** Alignment, Misalignment, Uncertainty, Reduced capacity, Intentional pause, Reassessment, Silence (neutral)

---

## Mental Model
```
Commit Store = canonical, append-only truth
Resolution Index = per-node view
Goal Influence Index = aggregated DAG view 
Area Metrics Index = operational forest view 
Derived Indexes = analytical layers (drift, tension, alignment horizon)
```
- Queries never mutate indexes
- Losing indexes = safe, rebuildable
- Supports both small-scale (V1) and large-scale (V6–V7) deployments
- Provides foundation for dashboards, AI integration, and monitoring frameworks
