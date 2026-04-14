# Charter — Implementation Tracking Board (v2)

Status: ACTIVE  
Purpose: Track substrate readiness, phase progression, and architectural integrity across implementation  

---

# 1. Phase Overview

| Phase | Name | Status | Notes |
|------|------|--------|------|
| 1 | Legitimacy Engine | IN PROGRESS | Core authority system |
| 2 | Runtime + Persistence Foundation | NOT STARTED | Orchestration + storage contracts |
| 3 | CLI (MVP) | NOT STARTED | Reference product |
| 4 | CCS | NOT STARTED | Commit protocol |
| 5 | Commit Store | NOT STARTED | Durable storage abstraction |
| 6 | CQL | NOT STARTED | Unified query interface (JSON IL) |
| 7 | CRS | NOT STARTED | Federation transport |
| 8 | CSG | NOT STARTED | Structural graph |
| 9 | CCare / CSP | NOT STARTED | Signals + feeds |
| 10 | Decision → Measurement Bridge | NOT STARTED | Observables + thresholds |
| 11 | CAS (v1) | NOT STARTED | Alignment computation |
| 12 | Federation Expansion | NOT STARTED | Commit taxonomy + policies |
| 13 | CIS / CRR | NOT STARTED | Identity + evolution |
| 14 | Disturbance Model | NOT STARTED | External interruption modeling |
| 15 | CAS Expansion | NOT STARTED | Cascade + simulation support |
| 16 | Simulation / Replay | NOT STARTED | CDS + CAS integration |
| 17 | Persistence Expansion | NOT STARTED | Full pluggable storage |
| 18 | VDS | NOT STARTED | Monitoring system |
| 19 | VLS | NOT STARTED | Lineage system |
| 20 | CGL | NOT STARTED | Guidance layer |

---

# 2. Substrate Status

## Legitimacy Engine
- Phase: 1
- Spec: FROZEN
- Implementation: IN PROGRESS
- Storage: None (stateless)
- FFI: Planned
- Notes: Deterministic authority system
- Risks: None (locked)

---

## Runtime
- Phase: 2
- Spec: IN DESIGN
- Implementation: NOT STARTED
- Storage: Interface-driven (in-memory + pluggable)
- FFI: Planned
- Notes:
  - owns sessions, reconciliation, orchestration
  - defines process model
  - integrates persistence contracts
- Risks:
  - scope creep
  - coupling to storage implementation

---

## Persistence Layer (Cross-Cutting)

- Phase: 2 (foundation), 17 (expansion)
- Spec: IN DESIGN
- Implementation: NOT STARTED
- Notes:
  - logical store contracts defined early
  - physical storage is host-provided
  - supports:
    - commit store
    - ref store
    - metadata store
    - workspace stores
- Risks:
  - premature backend assumptions
  - leaking storage concerns into runtime logic

---

## CDS (Deliberate System)
- Phase: 2+
- Spec: STABLE
- Implementation: PARTIAL
- Storage: Runtime-managed (pluggable)
- FFI: Planned
- Notes:
  - investigation layer
  - observation items used by VDS and simulation
- Risks:
  - mixing with legitimacy
  - unclear boundaries with replay

---

## CCS (Commit System)
- Phase: 4
- Spec: FROZEN
- Implementation: NOT STARTED
- Storage: Logical (via commit store)
- FFI: Planned
- Notes:
  - protocol for all durable artifacts
- Risks:
  - runtime leakage into commit semantics

---

## Commit Store
- Phase: 5
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Pluggable (append-only reference)
- FFI: Planned
- Notes:
  - canonical durable artifact layer
- Risks:
  - tight coupling to specific database types

---

## CQL (Query Layer)
- Phase: 6
- Spec: EVOLVING (JSON IL focus)
- Implementation: NOT STARTED
- Storage: N/A
- FFI: Planned
- Notes:
  - JSON Intermediate Language is canonical
  - DSL is human-only layer
  - used by CLI, Runtime, VDS, VLS
- Risks:
  - fragmentation across domains if not unified early

---

## CRS (Relay)
- Phase: 7
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Relay store (pluggable)
- FFI: Planned
- Notes:
  - commit transport only
  - no interpretation
- Risks:
  - hidden trust assumptions

---

## CSG (Structural Graph)
- Phase: 8
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Derived
- FFI: Planned
- Notes:
  - builds DAG from commits
  - supports simulation projections
- Risks:
  - overloading with meaning

---

## CCare
- Phase: 9
- Spec: EVOLVING
- Implementation: NOT STARTED
- Storage: Commit-backed (signals)
- FFI: Planned
- Notes:
  - signals originate from VDS
  - supports rich payloads
- Risks:
  - incomplete signal lifecycle model

---

## CSP (Signal Pipeline)
- Phase: 9
- Spec: EVOLVING
- Implementation: NOT STARTED
- Storage: Derived + pipeline state
- FFI: Planned
- Notes:
  - aggregation and shaping
  - outbound flow control
- Risks:
  - overengineering early

---

## Decision → Measurement Bridge
- Phase: 10
- Spec: IN DESIGN
- Implementation: NOT STARTED
- Notes:
  - observables as structure
  - thresholds as relationships
  - CDS observation items
- Risks:
  - implicit mapping (must not happen)

---

## CAS
- Phase: 11 (v1), 15 (expansion)
- Spec: EVOLVING
- Implementation: NOT STARTED
- Storage: Derived + commit summaries
- FFI: Planned
- Notes:
  - alignment computation
  - cascade modeling (later)
  - simulation support (later)
- Risks:
  - non-determinism
  - opaque outputs

---

## Federation Model (Cross-Cutting)

- Phase: 7+, 12
- Spec: EVOLVING
- Implementation: NOT STARTED
- Notes:
  - commit-centric push/fetch
  - CSP shapes outbound flow
  - CRS transports commits
- Risks:
  - inconsistent policies across systems

---

## CIS
- Phase: 13
- Spec: EVOLVING
- Implementation: NOT STARTED
- Storage: Commit-backed
- FFI: Planned
- Notes:
  - identity = bounded region of CSG
  - version = commit
- Risks:
  - boundary ambiguity
  - lack of standard encoding

---

## CRR
- Phase: 13
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Commit-backed
- FFI: Planned
- Notes:
  - promotion, demotion, split, merge
- Risks:
  - bypassing legitimacy (must not)

---

## Disturbance Model
- Phase: 14
- Spec: IN DESIGN
- Implementation: NOT STARTED
- Notes:
  - external interruption via commits
  - time windows + propagation
  - mapped to intentional pause signals
- Risks:
  - confusion with internal pauses

---

## Simulation / Replay
- Phase: 16
- Spec: IN DESIGN
- Implementation: NOT STARTED
- Notes:
  - uses CDS observation items
  - CAS computes cascade effects
  - no telemetry dependency required
- Risks:
  - accidental interpretation
  - unrealistic system assumptions

---

# 3. Product Status

## CLI (Reference Product)
- Phase: 3
- Status: NOT STARTED
- Scope: MVP
- Notes:
  - load → operate → flush model
  - context + area resolution
  - CQL-backed status commands
- Risks:
  - feature creep
  - bypassing runtime invariants

---

## VDS
- Phase: 18
- Status: NOT STARTED
- Notes:
  - signal origin system
  - threshold evaluation
  - CDS integration
- Risks:
  - depends on CCare, CAS, measurement bridge

---

## VLS
- Phase: 19
- Status: NOT STARTED
- Notes:
  - lineage + identity visualization
  - CAS + CIS integration
- Risks:
  - depends on identity + CAS maturity

---

# 4. Active Work (Now)

- [ ] Legitimacy Engine implementation
- [ ] Runtime architecture definition
- [ ] Persistence interface design
- [ ] Session lifecycle design
- [ ] Reconciliation workspace model

---

# 5. Next Up (Queue)

- [ ] Runtime implementation
- [ ] CLI interaction model
- [ ] CQL JSON IL draft
- [ ] CCS integration design
- [ ] Commit taxonomy (initial)

---

# 6. Key Architectural Decisions (Track)

- [x] Runtime-first operation
- [x] Dual representation (Runtime + CCS)
- [x] Reconciliation as only bridge
- [x] CQL as universal query interface
- [x] Persistence is interface-first and host-controlled
- [x] Commit-centric federation model
- [ ] Commit taxonomy (expand)
- [ ] Push/fetch policy model
- [ ] CAS computation boundaries

---

# 7. Transitional Flags

- minimal persistence implementations
- limited commit taxonomy
- no federation policies yet
- no simulation or replay yet

---

# 8. Risks and Watchpoints

- accidental legitimacy bypass
- runtime and CCS boundary blur
- persistence leaking into logic
- CAS becoming non-deterministic
- federation introducing hidden mutation
- over-designing simulation early

---

# 9. Notes

Use this section to track:
- implementation discoveries
- architecture changes
- deviations from plan