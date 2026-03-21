# ENG-AUD — Audit Event Specification

Status: REFACTORED (v4 – Pure Observability Model)  
Applies to: Engine Core (V1/V2+)  
Scope: Audit event structure, emission rules, and observability guarantees

Authority: Observability specification subordinate to ENG-API and ENG-INTEGRITY.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-SESSION
- ENG-DECISION
- ENG-SUPERSESSION
- ENG-RECEIPT
- ENG-ERROR
- ENG-CANON
- ENG-INITIALIZATION
- ENG-SPECVERIFY

---

# 1. Purpose

ENG-AUD defines the Engine’s audit event model.

It is the authoritative specification for:

- audit event structure
- audit emission rules
- audit identity and ordering
- audit scope classification
- guarantees of audit non-interference

Audit answers:

- what explicit Engine action occurred
- which structural objects were affected
- under which governance context it occurred

Audit does not:

- define legitimacy
- participate in evaluation
- affect structural state
- influence runtime behavior

---

# 2. Non-Authority Principle

## ENG-AUD-01 — Audit Is Observational Only

Audit events:

- do not create legitimacy
- do not modify structural state
- do not influence evaluation or acceptance
- do not participate in restore or rehydration
- do not affect supersession or ACTIVE derivation

The Engine must not:

- consult audit data for decision-making
- depend on audit presence for correctness
- derive ordering or precedence from audit

Removal or corruption of audit must not change:

- legitimacy outcomes
- structural validity
- runtime behavior

Audit is memory, not law.

---

# 3. Emission Model

## ENG-AUD-02 — Emission After Successful Mutation

Audit events are emitted only after successful Engine operations that mutate state.

Audit must not be emitted for:

- failed commands
- evaluation-only operations
- rejected acceptance attempts

Audit emission:

- occurs after the underlying operation succeeds
- must not be part of the atomic legitimacy boundary
- must not affect success or failure of the operation

Atomicity rules are defined in ENG-PERSISTENCE.  
Audit is outside that boundary.

---

## ENG-AUD-03 — Engine as Sole Producer

The Engine is the only producer of audit events.

The Engine must not:

- ingest audit as input
- reconstruct state from audit
- repair state using audit
- require audit for initialization

Audit is write-only from the Engine’s perspective.

---

# 4. Append-Only Guarantee

## ENG-AUD-04 — Immutable Event Log

Audit events are append-only.

The Engine must not:

- modify existing audit events
- delete audit events implicitly
- reorder historical audit entries

Corrections must be expressed as new events.

---

# 5. Scope Model

## ENG-AUD-05 — Event Scope Classification

Each audit event must belong to exactly one scope:

- GLOBAL  
- AREA:<area_id>  

Scope defines organizational grouping only.

Scope must not:

- affect legitimacy
- affect evaluation
- affect restore behavior

---

## ENG-AUD-06 — GLOBAL Scope Invariant

GLOBAL scope:

- must always exist
- must be immutable
- must not be superseded or removed

Used for:

- engine lifecycle events
- rehydration outcomes
- import/export operations
- system-level transitions

---

# 6. Audit Event Structure

## ENG-AUD-07 — Canonical Event Shape

Each audit event must include:

- event_id (UUIDv7)
- event_type
- occurred_at (UTC ISO-8601, informational)
- actor (nullable)
- scope
- subject
- context
- details

---

### Subject

- object_type  
- object_id  

---

### Context

Must include explicit nulls where not applicable:

- area_id  
- session_id  
- authority_resolution_id  
- scope_resolution_id  

---

### Details

- informational only  
- must not contain canonical domain objects  
- must not be required for structural reconstruction  

Event structure must be stable across implementations.

---

# 7. Identity & Ordering

## ENG-AUD-08 — Event Identity

Each event must have:

- globally unique UUIDv7 `event_id`

The Engine must not:

- reuse event IDs
- derive meaning from ID ordering beyond deterministic export

---

## ENG-AUD-09 — Timestamp Semantics

`occurred_at`:

- must be UTC ISO-8601
- is informational only

Timestamps must not:

- influence legitimacy
- influence ordering for evaluation
- influence supersession or acceptance

---

## ENG-AUD-10 — Deterministic Export Ordering

Audit export must be deterministic.

Ordering must be:

- lexicographic by `event_id`

Ordering must not depend on:

- timestamps
- storage order
- insertion order

---

# 8. Auditable Actions

## ENG-AUD-11 — Emission Coverage

Audit must be emitted for successful Engine operations including:

- session lifecycle transitions
- participant changes
- candidate changes
- vote recording
- acceptance and closure
- resolution creation and supersession
- administrative state transitions (UNDER_REVIEW / ACTIVE)
- governance slot changes
- rehydration outcomes
- degraded mode entry
- import/export operations

The exact semantics of these actions are defined elsewhere.

ENG-AUD defines only that:

- successful state-changing operations must produce audit events
- audit must reflect the action that occurred

---

# 9. Actor Semantics

## ENG-AUD-12 — Actor Is Informational

`actor`:

- is optional
- is opaque
- does not confer authority

The Engine must not:

- derive permissions from actor
- use actor in evaluation logic

---

# 10. Relationship to Receipts

## ENG-AUD-13 — Audit vs Receipt

Receipts:

- are structural artifacts
- define legitimacy outcomes

Audit:

- records that an action occurred

Audit must not:

- substitute for receipts
- imply legitimacy without corresponding domain objects

If audit and domain objects disagree, domain objects prevail.

---

# 11. Runtime Independence

## ENG-AUD-14 — Rehydration Independence

Initialization must not:

- require audit presence
- validate audit structure
- depend on audit ordering

Rehydration rules are defined in ENG-INITIALIZATION and ENG-INTEGRITY.

---

## ENG-AUD-15 — Degraded Mode Behavior

In degraded mode:

- audit may continue to append events (implementation-dependent)
- audit must not enable mutation or legitimacy creation
- audit must not compensate for integrity failures

Degraded mode rules are defined in ENG-INTEGRITY.

---

# 12. Storage Independence

## ENG-AUD-16 — Audit Persistence Independence

Audit storage:

- is independent of domain object storage
- must not affect legitimacy or restore

The Engine must not:

- fail initialization due to missing audit
- reconstruct audit from domain objects
- reconstruct domain objects from audit

---

# 13. Determinism Guarantees

## ENG-AUD-17 — Deterministic Emission

Given identical Engine operations:

- identical audit events must be emitted
- event structure must be identical
- ordering must be identical for export

Audit must not introduce nondeterminism.

---

# 14. Engine Invariants (Audit Layer)

- Audit never creates legitimacy
- Audit never influences evaluation
- Audit never alters structural state
- Audit never participates in restore
- Audit never substitutes for receipts
- Audit never encodes hidden rules
- Audit ordering never affects outcomes

Violation constitutes an observability defect, not a legitimacy rule.

---

# 15. Mental Model

Audit is the Engine’s observable trace.

- Sessions create legitimacy
- Resolutions define structure
- Receipts freeze outcomes
- Integrity governs safety
- API exposes behavior

Audit records what happened.

It does not decide what is valid.