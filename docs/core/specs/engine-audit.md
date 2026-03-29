# ENG-AUD — Audit Event Specification

Status: REFACTORED (v5 – Candidate-Aware, Structure-Aligned Observability Model)  
Applies to: Engine Core (V1/V2+)  
Scope: Audit event structure, emission rules, and observability guarantees

Authority: Observability specification subordinate to ENG-API and ENG-INTEGRITY.

Subordinate references consumed from:

- ENG-DOMAIN  
- ENG-SESSION  
- ENG-DECISION  
- ENG-STRUCTURE  
- ENG-RECEIPT  
- ENG-ERROR  
- ENG-CANON  
- ENG-INITIALIZATION  
- ENG-SPECVERIFY  
- ENG-IMPORT  
- ENG-COMPILATION  

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
- participate in historical reconstruction  

---

# 2. Non-Authority Principle

## ENG-AUD-01 — Audit Is Observational Only

Audit events:

- do not create legitimacy  
- do not modify structural state  
- do not influence evaluation or acceptance  
- do not participate in restore, import, or compilation  
- do not affect structural ACTIVE derivation  

The Engine must not:

- consult audit data for decision-making  
- depend on audit presence for correctness  
- derive ordering or precedence from audit  

Removal or corruption of audit must not change:

- legitimacy outcomes  
- structural validity  
- runtime behavior  
- compilation results  

Audit is memory, not authority.

---

# 3. Emission Model

## ENG-AUD-02 — Emission After Successful Mutation

Audit events are emitted only after successful state mutation.

Audit must not be emitted for:

- failed commands  
- evaluation-only operations  
- rejected acceptance attempts  

Audit emission:

- occurs strictly after mutation succeeds  
- must not be part of any atomic persistence boundary  
- must not affect mutation success or failure  

Atomicity is defined in ENG-PERSISTENCE.

---

## ENG-AUD-03 — Engine as Sole Producer

The Engine is the only producer of audit events.

The Engine must not:

- ingest audit as input  
- reconstruct state from audit  
- repair state using audit  
- require audit for initialization or rehydration  

Audit is write-only from the Engine perspective.

---

# 4. Append-Only Guarantee

## ENG-AUD-04 — Immutable Event Log

Audit events are append-only.

The Engine must not:

- modify existing events  
- delete events implicitly  
- reorder events  

Corrections must be expressed as new events.

---

# 5. Scope Model

## ENG-AUD-05 — Event Scope Classification

Each event must belong to exactly one scope:

- GLOBAL  
- AREA:<area_id>  

Scope defines grouping only.

Scope must not affect:

- legitimacy  
- evaluation  
- restore or compilation behavior  

---

## ENG-AUD-06 — GLOBAL Scope Invariant

GLOBAL scope:

- must always exist  
- must be immutable  

Used for:

- engine lifecycle events  
- rehydration results  
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

Must support all domain object types, including:

- session  
- candidate  
- resolution  
- participant  

---

### Context

Must include explicit nulls where not applicable:

- area_id  
- session_id  
- authority_resolution_id  
- scope_resolution_id  

---

### Details

Details are strictly informational.

They must not:

- contain canonical domain objects  
- duplicate full structural payloads  
- be required for reconstruction  
- affect evaluation or legitimacy  

Details may include:

- human-readable summaries  
- lightweight contextual metadata  
- identifiers already present elsewhere  

---

# 7. Identity & Ordering

## ENG-AUD-08 — Event Identity

Each event must have a globally unique UUIDv7.

Event identity must not encode semantic meaning.

---

## ENG-AUD-09 — Timestamp Semantics

`occurred_at`:

- is informational only  
- must not influence ordering, evaluation, or legitimacy  

---

## ENG-AUD-10 — Deterministic Export Ordering

Audit export ordering must be:

- lexicographic by event_id  

Ordering must not depend on:

- timestamps  
- storage order  
- insertion order  

---

# 8. Auditable Actions

## ENG-AUD-11 — Emission Coverage

Audit must be emitted for successful mutations including:

- session lifecycle transitions  
- participant changes  
- candidate creation and mutation  
- vote updates (including vacillation)  
- acceptance and closure  
- resolution creation and supersession  
- usability transitions (ACTIVE ↔ ON_HOLD, RETIRED)  
- governance slot changes  
- rehydration outcomes  
- degraded mode entry  
- import and export operations  

Audit must reflect:

- the action performed  
- the affected object  

Audit must not infer or interpret outcomes.

---

# 9. Candidate Model Alignment

## ENG-AUD-12 — Candidate-Centric Observability

Audit must support candidate-level visibility.

This includes:

- candidate creation  
- candidate updates  
- candidate participation in rounds  

Audit may record context such as:

- candidate involvement in acceptance attempts  
- candidate-level blocking visibility  

Audit must not determine candidate eligibility.

Eligibility is defined in ENG-DECISION.

---

# 10. Actor Semantics

## ENG-AUD-13 — Actor Is Informational Only

`actor`:

- is optional  
- is opaque  
- does not confer authority  

The Engine must not:

- derive permissions from actor  
- use actor in evaluation  

---

# 11. Relationship to Receipts

## ENG-AUD-14 — Audit vs Receipt

Receipts:

- define legitimacy  

Audit:

- records actions  

If conflict exists:

- receipts and domain objects prevail  

Audit must never imply legitimacy.

---

# 12. Runtime Independence

## ENG-AUD-15 — Rehydration Independence

Initialization must not:

- require audit  
- validate audit  
- depend on audit ordering  

---

## ENG-AUD-16 — Degraded Mode

In degraded mode:

- audit may continue  
- audit must not enable mutation  
- audit must not compensate for failures  

---

# 13. Storage Independence

## ENG-AUD-17 — Audit Storage Separation

Audit storage is independent.

The Engine must not:

- fail due to missing audit  
- reconstruct audit from domain objects  
- reconstruct domain objects from audit  

---

# 14. Determinism Guarantees

## ENG-AUD-18 — Deterministic Emission

Given identical operations:

- event_type must match  
- subject must match  
- context must match  
- details structure must match  

The following may differ:

- event_id  
- occurred_at  

Audit must not introduce nondeterminism into Engine behavior.

---

# 15. Engine Invariants

- audit never creates legitimacy  
- audit never affects evaluation  
- audit never alters structure  
- audit never participates in restore or compilation  
- audit never substitutes for receipts  
- audit never encodes hidden rules  
- audit ordering never affects outcomes  

---

# 16. Mental Model

Audit is the observable trace.

- sessions create legitimacy  
- candidates carry intent  
- structure defines graph truth  
- receipts finalize outcomes  

Audit records what happened.

It does not decide what is valid.