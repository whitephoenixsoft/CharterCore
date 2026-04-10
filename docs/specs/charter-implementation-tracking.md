# Charter — Implementation Tracking Board

Status: ACTIVE  
Purpose: Track substrate readiness, phase progression, and architectural integrity across implementation  

---

# 1. Phase Overview

| Phase | Name | Status | Notes |
|------|------|--------|------|
| 1 | Legitimacy Engine | IN PROGRESS | Core authority system |
| 2 | Runtime | NOT STARTED | Orchestration + reconciliation |
| 3 | CLI (MVP) | NOT STARTED | Reference product |
| 4 | CCS | NOT STARTED | Commit protocol |
| 5 | Commit Store | NOT STARTED | Durable storage |
| 6 | CRS | NOT STARTED | Federation |
| 7 | CSG | NOT STARTED | Structural graph |
| 8 | CCare / CSP | NOT STARTED | Signals + feeds |
| 9 | CAS | NOT STARTED | Alignment |
| 10 | CQL / CGL | NOT STARTED | Query + guidance |
| 11 | CIS / CRR | NOT STARTED | Identity + evolution |
| 12 | Products (VDS / VLS) | NOT STARTED | Product layer |

---

# 2. Substrate Status

## Legitimacy Engine
- Phase: 1
- Spec: FROZEN
- Implementation: IN PROGRESS
- Storage: Runtime-local
- FFI: Planned
- Notes: Stateless, deterministic core
- Risks: None (locked)

---

## Runtime
- Phase: 2
- Spec: IN DESIGN
- Implementation: NOT STARTED
- Storage: Runtime-local (transitional)
- FFI: Planned
- Notes: Owns sessions, reconciliation, orchestration
- Risks: Scope creep, accidental coupling

---

## CDS (Deliberate)
- Phase: 2 (Runtime dependency)
- Spec: STABLE
- Implementation: PARTIAL
- Storage: Runtime-local
- FFI: Planned
- Notes: Investigation layer
- Risks: Mixing with legitimacy if not careful

---

## CCS (Commit System)
- Phase: 4
- Spec: FROZEN
- Implementation: NOT STARTED
- Storage: Local (initial), Commit Store (later)
- FFI: Planned
- Notes: Artifact protocol, not workflow
- Risks: Runtime leakage into CCS

---

## Commit Store
- Phase: 5
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Append-only (canonical)
- FFI: Planned
- Notes: Durable artifact history
- Risks: Premature dependency

---

## CRS (Relay)
- Phase: 6
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Relay store
- FFI: Planned
- Notes: Transport only
- Risks: Implicit trust assumptions

---

## CSG (Structural Graph)
- Phase: 7
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Derived
- FFI: Planned
- Notes: DAG from commits
- Risks: Overloading semantics

---

## CCare
- Phase: 8
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Commit-backed
- FFI: Planned
- Notes: Signals + check-ins
- Risks: Premature UI design

---

## CSP (Feeds)
- Phase: 8
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Commit-triggered
- FFI: Planned
- Notes: Subscription + notification layer
- Risks: Overengineering early

---

## CAS
- Phase: 9
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Derived
- FFI: Planned
- Notes: Alignment detection
- Risks: Depends on good structure

---

## CQL
- Phase: 10
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: N/A
- FFI: Planned
- Notes: Query layer
- Risks: Over-expansion

---

## CGL
- Phase: 10
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: N/A
- FFI: Planned
- Notes: Explanation + assistance
- Risks: Overstepping authority

---

## CIS
- Phase: 11
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Commit-backed
- FFI: Planned
- Notes: Identity + scope
- Risks: Premature modeling

---

## CRR
- Phase: 11
- Spec: STABLE
- Implementation: NOT STARTED
- Storage: Commit-backed
- FFI: Planned
- Notes: Structural evolution via reconciliation
- Risks: Bypassing legitimacy (must not)

---

# 3. Product Status

## CLI (Reference Product)
- Phase: 3
- Status: NOT STARTED
- Scope: MVP only
- Notes:
  - bootstrap
  - decision flow
  - reconciliation
  - audit
- Risks: Becoming too feature-heavy

---

## VDS
- Phase: 12
- Status: NOT STARTED
- Notes: Care-focused product
- Risks: Requires CCare + CAS maturity

---

## VLS
- Phase: 12
- Status: NOT STARTED
- Notes: Identity-focused product
- Risks: Requires CIS + CRR maturity

---

# 4. Active Work (Now)

- [ ] Legitimacy Engine implementation
- [ ] Runtime architecture definition
- [ ] Session lifecycle design
- [ ] Reconciliation workspace model
- [ ] Local storage boundaries

---

# 5. Next Up (Queue)

- [ ] Runtime spec (detailed)
- [ ] CLI interaction model
- [ ] CCS integration design
- [ ] Commit emission rules

---

# 6. Key Architectural Decisions (Track)

- [x] Runtime-first operation
- [x] Dual-write (Runtime + CCS)
- [x] Reconciliation as only bridge
- [x] Explicit CGL invocation
- [ ] Commit emission timing (refine)
- [ ] Runtime → Commit Store migration strategy

---

# 7. Transitional Flags

These indicate temporary implementations that must be revisited:

- Runtime-local storage (no Commit Store yet)
- No CCS integration yet
- Simplified import/export
- No relay/federation

---

# 8. Risks & Watchpoints

- Accidental legitimacy bypass
- Runtime/CCS boundary blur
- Premature product complexity
- Hardening temporary storage
- Over-designing CAS/CGL early

---

# 9. Notes

Use this section to track:
- implementation discoveries
- architecture changes
- deviations from plan

---