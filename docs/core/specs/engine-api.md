# ENG-API — Engine Interface & Execution Boundary Specification

Status: REFACTORED (v15.1 – Authority-Aligned Interface Model, Persistence-Aligned)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic Engine interface, command surface, and runtime interaction boundary

Authority: Interface specification subordinate to ENG-INITIALIZATION and ENG-INTEGRITY.

Subordinate references consumed from:

- ENG-DOMAIN
- ENG-SESSION
- ENG-DECISION
- ENG-SUPERSESSION
- ENG-RECEIPT
- ENG-CANON
- ENG-ERROR
- ENG-SPECVERIFY
- ENG-INITIALIZATION
- ENG-IMPORT
- ENG-COMPILATION
- ENG-PERSISTENCE

---

# 1. Purpose

ENG-API defines the deterministic interface between a host system and the Engine Core.

It is the authoritative specification for:

- command surface exposed by the Engine
- runtime interaction model
- mutation vs evaluation boundaries
- rehydration entry invocation
- incremental compilation invocation
- deterministic reporting guarantees

ENG-API does not define:

- structural validation rules
- session mechanics
- acceptance rules
- supersession semantics
- receipt structure
- canonical serialization rules
- specification identity semantics

Those are defined in:

- ENG-INTEGRITY
- ENG-SESSION
- ENG-DECISION
- ENG-SUPERSESSION
- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY

ENG-API exposes these capabilities.  
It does not redefine them.

---

# 2. Core Principles

## ENG-API-01 — Interface, Not Authority

The API provides access to Engine behavior.

It must not:

- redefine validation logic
- bypass governing specifications
- introduce alternative execution paths
- encode hidden legitimacy rules

All behavior is delegated to authoritative specifications.

---

## ENG-API-02 — Single-Area Runtime Model

The Engine operates on exactly one Area at a time.

- Runtime state exists only after successful initialization
- Area switching occurs only via rehydration
- No cross-Area evaluation is permitted

This invariant is enforced by:

- ENG-INITIALIZATION
- ENG-INTEGRITY

---

## ENG-API-03 — Deterministic Reporting

All mutating commands must produce exactly one EvaluationReport.

EvaluationReports:

- are defined in ENG-ERROR
- must be deterministic
- must contain ordered errors
- must not encode meaning through exceptions or side channels

The Engine must not:

- throw semantic exceptions
- return boolean legitimacy flags
- short-circuit validation

---

## ENG-API-04 — Identifier Ownership

All identifiers are Engine-generated UUIDv7 values.

This includes:

- session_id
- participant_id
- candidate_id
- resolution_id
- receipt_id
- vote_id

The API must not allow caller-supplied identifiers for Engine objects.

Identifier semantics are defined in ENG-DOMAIN and ENG-SESSION.

---

## ENG-API-05 — No Implicit Legitimacy

The API must not provide any operation that:

- creates legitimacy outside session acceptance
- directly mutates Resolution state (except administrative transitions defined elsewhere)
- bypasses governance validation
- reconstructs legitimacy artifacts

Legitimacy creation is governed exclusively by ENG-DECISION and committed via ENG-PERSISTENCE.

---

# 3. Runtime Entry

## ENG-API-06 — rehydrate_engine

Rehydrates the Engine from a host-provided domain graph.

Inputs:

- domain objects (sessions, resolutions, receipts, annotations)

Behavior:

- Delegates to ENG-INITIALIZATION
- Executes structural validation via ENG-INTEGRITY
- Verifies receipts via ENG-RECEIPT + ENG-CANON + ENG-SPECVERIFY
- Reconstructs supersession graph via ENG-SUPERSESSION
- Establishes runtime mode

Possible outcomes:

- NORMAL_RUNTIME
- DEGRADED_READ_ONLY
- INITIALIZATION_FAILURE

ENG-API invokes runtime entry.  
It does not define validation rules.

---

# 4. Evaluation Interface

## ENG-API-07 — evaluate_session

Evaluates a session without mutation.

Properties:

- pure
- deterministic
- idempotent
- side-effect-free

Behavior:

- Delegates to ENG-DECISION evaluation logic
- Executes full validation pass
- Produces EvaluationReport

Must not:

- mutate session state
- emit receipts
- trigger transitions
- insert implicit votes

Evaluation is simulation.  
Acceptance is execution coordinated with ENG-PERSISTENCE.

---

# 5. Session Lifecycle Commands

## ENG-API-08 — Lifecycle Commands Are Delegations

All session lifecycle operations:

- delegate to ENG-SESSION (structure and lifecycle)
- delegate to ENG-DECISION (validation and acceptance logic)
- rely on ENG-INTEGRITY for safety guarantees

Each operation:

- executes a full validation pass
- produces an EvaluationReport
- mutates state only on successful validation

No partial mutation is permitted.

Acceptance behavior (including resolution creation, supersession, and receipt emission) is defined in:

- ENG-DECISION
- ENG-SESSION
- ENG-RECEIPT
- ENG-SUPERSESSION
- ENG-PERSISTENCE

---

# 6. Incremental Compilation Interface

## ENG-API-09 — Incremental Compilation Commands

The API exposes incremental compilation operations:

- begin_incremental_compilation  
- add_incremental_batch  
- end_incremental_compilation  

These operations:

- delegate to ENG-COMPILATION for replay and ordering
- rely on ENG-INTEGRITY for structural validation
- rely on ENG-SUPERSESSION for graph integration

During incremental compilation:

- runtime mutation operations must not proceed
- legitimacy creation via standard session acceptance is suspended

ENG-API does not define replay or conflict rules.  
It exposes compilation entry points only.

---

# 7. Administrative Lifecycle Operations

## ENG-API-10 — Administrative State Transitions

Administrative operations such as:

- set_resolution_under_review  
- restore_resolution_active  

are exposed through the API.

Their semantics are defined in:

- ENG-REVIEW-RETIRED

These operations:

- do not create legitimacy
- do not alter historical legitimacy
- do not bypass structural validation

The API must emit deterministic EvaluationReports.

---

# 8. Read-Only Queries

## ENG-API-11 — Query Interface

The API provides read-only access to Engine state.

Examples:

- list_sessions  
- list_resolutions  
- get_session_state  
- get_resolution_state  
- get_session_receipt  
- list_session_receipts  

Query responses must be:

- deterministic
- derived from current runtime state
- consistent with canonical structures

Receipt fields are defined in ENG-RECEIPT.

---

# 9. Specification Identity Interface

## ENG-API-12 — Spec Verification Access

The API exposes rule identity:

- get_spec_set_hash  
- verify_spec_hash  

Semantics defined in:

- ENG-SPECVERIFY

The API must not reinterpret rule identity.  
It exposes it.

---

# 10. Export Interface

## ENG-API-13 — export_area_dag

Exports the current Area graph.

The exported graph must:

- be deterministic
- be suitable for rehydration
- preserve canonical structure

Export semantics rely on:

- ENG-DOMAIN
- ENG-CANON
- ENG-RECEIPT

---

# 11. Degraded Mode Behavior

## ENG-API-14 — Read-Only Enforcement

When runtime mode is DEGRADED_READ_ONLY:

- mutating commands must fail deterministically
- read-only operations remain available

Degraded mode semantics are defined in:

- ENG-INTEGRITY
- ENG-INITIALIZATION

ENG-API enforces, but does not define, degraded behavior.

---

# 12. Determinism Guarantees

## ENG-API-15 — Deterministic Interface Behavior

Given identical inputs and runtime state:

- identical commands produce identical EvaluationReports
- identical queries produce identical results
- no behavior depends on timestamps, ordering, or environment

Determinism is enforced by underlying specifications.

---

# 13. Engine Invariants (Interface-Level)

- API never creates legitimacy directly
- API never bypasses validation
- API never mutates state on failed validation
- API never exposes partial acceptance
- API never evaluates without rehydration
- API never merges Areas
- API never reuses identifiers
- API never interprets rule identity differently than ENG-SPECVERIFY

---

# 14. Mental Model

ENG-API is the execution surface.

It answers:

- how a host interacts with the Engine
- how commands are issued
- how results are reported
- how runtime entry is triggered
- how evaluation differs from mutation

It does not answer:

- how sessions work
- how legitimacy is determined
- how receipts are constructed
- how supersession evolves
- how structural validity is enforced

Those belong elsewhere.

ENG-API exposes the Engine.  
It does not define it.