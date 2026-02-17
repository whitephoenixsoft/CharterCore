# Charter Multi-Layer Structural Design

**Purpose:** Document the layered architecture of Charter, including engines, libraries, and observational layers, preserving intent, system relationships, and auditable flows. No solutions; purely structural.

---

## 1. Introduction
- **1.1 Purpose of this document**  
  Explain the structural organization and relationships of Charter layers (V1–V7).
- **1.2 Scope**  
  Structural overview of Engine (V1/V2), CLI Library (V4), Guidance (V5), Federation (V6), and Commit Relay (V7).
- **1.3 Audience**  
  Future maintainers, developers, and system architects.
- **1.4 Intended use**  
  Reference for implementation, dependency management, and evolution.

---

## 2. Charter Architecture Overview
- **2.1 Layered Model**
  - Engine layer (V1/V2)  
  - CLI orchestration library (V4)  
  - Guidance layer (V5)  
  - Federation layer (V6)  
  - Commit relay layer (V7)
- **2.2 Dependency direction and boundaries**  
  Lower layers provide truth; higher layers observe or consume.
- **2.3 Layer interaction principles**
  - No upstream mutation of legitimacy.
  - Audit and history are append-only.
  - Observational layers may read any lower-layer state.
- **2.4 System mental model summary**  
  Each layer is responsible for a bounded concern; feedback loops are read-only.

---

## 3. Engine Layer (V1/V2)
- **3.1 Core responsibilities**
  - Deterministic legitimacy creation.
  - Session and resolution lifecycle management.
  - Authority, scope, and session constraint enforcement.
  - Audit event emission.
- **3.2 Input/output interfaces**  
  CLI commands → engine → audit events.
- **3.3 Concurrency, isolation, and session invariants**  
  Single active session per legitimacy context; paused or blocked sessions allowed.
- **3.4 Feedback relationships**  
  Engine is foundational; no upstream feedback occurs.

---

## 4. CLI Library Layer (V4)
- **4.1 Core responsibilities**
  - Orchestrates sessions, candidates, and baseline review.
  - Calls engine for legitimacy evaluation.
  - Produces structured outputs for higher layers.
- **4.2 Dependencies and API contracts**
  - Requires Engine (V1/V2).
  - Provides data consumable by V5–V7.
- **4.3 State management principles**
  - Session lifecycle: ACTIVE, PAUSED, BLOCKED, CLOSED.
  - Context preservation for auditability.
- **4.4 Observational paths**
  - Audit, guidance, and federation consume CLI outputs without mutating engine state.

---

## 5. Guidance Layer (V5)
- **5.1 Responsibilities**
  - Observes engine and CLI state.
  - Generates read-only summaries, drift reports, coherence feedback.
  - Does not create legitimacy or alter engine state.
- **5.2 Inputs**  
  CLI and engine states, sessions, resolutions.
- **5.3 Outputs**  
  Human-readable or machine-readable guidance.
- **5.4 Observational feedback loops**  
  Advisory only; no upstream mutation permitted.

---

## 6. Federation Layer (V6)
- **6.1 Responsibilities**
  - Observes multiple areas or team instances.
  - Aggregates signals (check-ins, capacity, intent).
  - Summarizes for reflection or discussion.
- **6.2 Dependencies**
  - CLI library is the canonical data source.
  - Optional consumption by V5 for broader insight.
- **6.3 Concurrency and aggregation principles**
  - Aggregated signals do not alter underlying legitimacy.
- **6.4 Audit alignment**  
  Federation outputs are fully auditable.

---

## 7. Commit Relay Layer (V7)
- **7.1 Responsibilities**
  - Provides append-only commit relay.
  - Stores all commit types: resolutions, deliberates, baselines, annotations.
  - Supports multi-consumer read access.
- **7.2 Dependencies**
  - CLI library as the authoritative source.
- **7.3 Operational constraints**
  - Immutable commits.
  - No legitimacy assignment.
  - Manual push/fetch only.
- **7.4 Audit alignment**
  - Relay events are auditable and traceable.

---

## 8. Inter-Layer Relationships
- **8.1 Dependency graph**  
  V1/V2 → V4 → {V5, V6, V7}.
- **8.2 Data flow and call paths**
  - Engine emits audit events.
  - CLI library orchestrates sessions and produces structured data.
  - Higher layers read state and provide insight without writing legitimacy.
- **8.3 Feedback loops**
  - Read-only observation from V5, V6, V7.
- **8.4 Cross-layer invariants and restrictions**
  - Only engine mutates legitimacy.
  - Higher layers may not mutate sessions, resolutions, authority, or scope.

---

## 9. Invariants Across Layers
- Engine invariants (V1/V2) are the source of truth.
- CLI orchestration preserves session and resolution integrity.
- Guidance, federation, and relay layers cannot mutate engine state.
- Audit and append-only guarantees enforced across all layers.

---

## 10. Deployment Considerations
- **10.1 Library independence and versioning**
  - Each layer can be independently versioned.
- **10.2 Multi-instance scenarios**
  - Multiple CLIs, federation consumers, and guidance consumers may coexist.
- **10.3 Layer isolation for correctness testing**
  - Lower layers tested in isolation; higher layers observed in integration.
- **10.4 Observability and monitoring points**
  - All layers emit audit/log events for traceability.

---

## 11. Conclusion
- Recap: layered, independent library design ensures correctness, auditability, and separation of concerns.
- Structural emphasis on:
  - Engine as truth source.
  - CLI library as orchestrator.
  - Guidance, federation, and relay as read-only observation layers.
- Future evolution can add layers, new queries, or guidance mechanisms without violating engine correctness.