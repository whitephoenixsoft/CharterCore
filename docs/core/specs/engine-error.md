# ENG-ERROR — Engine Error & EvaluationReport Model

Status: REFACTORED (v13 – Full Determinism Closure, Candidate & Structure Alignment)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic reporting, EvaluationReport structure, error classification, and canonical output  

Authority: Foundational authority for Engine error representation and EvaluationReport semantics.

Subordinate references consumed from:

- ENG-INTEGRITY  
- ENG-INITIALIZATION  
- ENG-DECISION  
- ENG-SESSION  
- ENG-STRUCTURE  
- ENG-USABILITY  
- ENG-RECEIPT  
- ENG-CANON  
- ENG-SPECVERIFY  
- ENG-API  

---

# 1. Purpose

ENG-ERROR defines the Engine’s deterministic reporting contract.

It is the authoritative specification for:

- EvaluationReport structure  
- ErrorEntry structure  
- outcome classification  
- ordered error accumulation rules  
- deterministic report ordering  
- canonical EvaluationReport representation  
- stable and closed error code vocabulary  

ENG-ERROR does not define:

- structural validity rules → ENG-STRUCTURE / ENG-INTEGRITY  
- usability semantics → ENG-USABILITY  
- decision logic → ENG-DECISION  
- session lifecycle → ENG-SESSION  
- receipt structure or validity → ENG-RECEIPT  

ENG-ERROR defines how all other specifications surface their outcomes.

---

# 2. Core Reporting Principle

## ENG-ERROR-01 — Deterministic Structured Reporting

Every command-level operation must produce exactly one EvaluationReport.

Requirements:

- no silent failures  
- no semantic meaning in free-form strings  
- all violations machine-readable  
- no short-circuit evaluation  
- identical inputs produce identical report semantics  

EvaluationReport applies to command-level operations.

Initialization, import, and compilation processes may produce outcomes outside this model.  
When surfaced through ENG-API, they must be represented using this EvaluationReport structure and error vocabulary.

---

# 3. Evaluation Model

## ENG-ERROR-02 — Full Evaluation Before Outcome

The Engine must:

- execute a complete validation pass  
- accumulate all violations  
- derive outcome only after evaluation completes  

The Engine must not:

- stop on first error  
- infer outcome early  
- omit applicable violations  

All violations detectable under governing specifications must be reported.

Each EvaluationReport has exactly one outcome:

- SUCCESS  
- REJECTED  
- BLOCKED  
- NO_OP  

---

## ENG-ERROR-03 — Evaluation Is Pure

Evaluation must be side-effect free.

Evaluation must not:

- mutate state  
- emit receipts  
- emit audit events  
- trigger lifecycle transitions  

Evaluation inspects structural truth and produces a report only.

---

# 4. Evaluation Phase Ordering

## ENG-ERROR-04 — Deterministic Violation Ordering

Violations must be ordered by reporting phase:

1. Structural validation  
2. Receipt validation  
3. Governance validation  
4. Session state validation  
5. Freeze boundary validation  
6. Participant validation  
7. Resolution lifecycle validation  
8. Decision / constraint validation  
9. Blocking evaluation  

Reporting phases are logical groupings for deterministic ordering.  
They do not depend on execution order.

Within each phase:

- errors sorted lexicographically by error_code  
- then lexicographically by related object identifiers  

block_scope must not affect ordering.

---

# 5. EvaluationReport Schema

## ENG-ERROR-05 — Fixed Report Shape

Fields in exact order:

1. evaluation_id  
2. command_type  
3. target_object_type  
4. target_object_id  
5. outcome  
6. errors  
7. primary_error_code  
8. diagnostics  
9. occurred_at  
10. schema_version  

Rules:

- no field reordering  
- optional fields must be explicit null  
- deterministic serialization required  

---

## ENG-ERROR-06 — ErrorEntry Structure

Each ErrorEntry contains:

- error_code  
- related_objects  
- block_type  
- block_scope  

### block_type

- TEMPORARY  
- PERMANENT  
- null  

### block_scope

- SESSION  
- CANDIDATE  
- null  

Rules:

- block_scope must be present when block_type is non-null  
- related_objects sorted lexicographically  
- each error_code appears at most once  

---

# 6. Outcome Derivation

## ENG-ERROR-07 — Outcome Precedence

Outcome is derived after full evaluation:

1. Any structural violation → REJECTED  
2. Any hard invariant violation → REJECTED  
3. Any blocking condition → BLOCKED  
4. Mutation occurred → SUCCESS  
5. Otherwise → NO_OP  

Mutation must be determined from command semantics, not from evaluation side effects.

---

# 7. Blocking Model (Reporting Layer)

## ENG-ERROR-08 — Dual-Level Blocking Representation

Blocking must be explicitly represented.

### Session-Level Blocking

- block_scope = SESSION  
- affects entire session  

### Candidate-Level Blocking

- block_scope = CANDIDATE  
- affects individual candidates  

Blocking classification is defined by:

- ENG-DECISION  
- ENG-USABILITY  
- ENG-STRUCTURE  

ENG-ERROR represents blocking only.

---

# 8. Error Code Vocabulary

## ENG-ERROR-09 — Closed Enumerated Codes

Error codes are:

- explicit  
- stable  
- deterministic  
- closed for a given schema_version  

Implementations must not introduce additional error codes.

---

## 8.1 Structural Errors

- INVALID_UUID  
- DUPLICATE_ID  
- MISSING_REFERENCE  
- ORPHAN_REFERENCE_DETECTED  
- INVALID_ENUM_VALUE  
- INVALID_STATE_COMBINATION  
- STRUCTURE_CYCLE_DETECTED  
- MULTI_AREA_GRAPH_DETECTED  
- CROSS_AREA_STRUCTURE_EDGE_PROHIBITED  
- GOVERNANCE_SLOT_EMPTY  
- GOVERNANCE_SLOT_MULTIPLICITY  
- PARTICIPANT_ID_REUSE_DETECTED  

Receipt-related:

- RECEIPT_MISSING  
- RECEIPT_HASH_MISMATCH  
- RECEIPT_ORPHAN_DETECTED  
- SNAPSHOT_PARTICIPANT_MISMATCH  

---

## 8.2 Runtime Mode

- DEGRADED_MODE_ACTIVE  

---

## 8.3 Session State Errors

- SESSION_TERMINAL_IMMUTABLE  
- SESSION_NOT_ACTIVE  
- SESSION_BLOCKED_TEMPORARY  
- SESSION_BLOCKED_PERMANENT  

---

## 8.4 Freeze Boundary Errors

- CANDIDATE_SET_FROZEN  
- PARTICIPANT_SET_FROZEN  
- CONSTRAINT_MUTATION_FORBIDDEN  

---

## 8.5 Participant Errors

- PARTICIPANT_NOT_FOUND  
- CANNOT_REMOVE_LAST_PARTICIPANT  
- DUPLICATE_PARTICIPANT_DISPLAY_NAME  
- PARTICIPANT_ALREADY_REMOVED  
- INVALID_PARTICIPANT_EPOCH  

---

## 8.6 Governance Errors

- AUTHORITY_CONTEXT_MISMATCH  
- SCOPE_CONTEXT_MISMATCH  

---

## 8.7 Decision & Acceptance Errors

- ACCEPTANCE_CONDITIONS_NOT_MET  
- AREA_BLOCKED_BY_PERMANENT_SESSION  
- AUTHORITY_RULE_VIOLATION  
- CONSTRAINT_VIOLATION  
- STRUCTURAL_CONFLICT  
- RESOLUTION_ALREADY_SUPERSEDED  

---

## 8.8 Usability & Lifecycle Errors

- INVALID_RESOLUTION_STATE_TRANSITION  
- RETIRED_STATE_VIOLATION  
- ON_HOLD_STATE_VIOLATION  
- SNAPSHOT_INCOMPLETE  

---

# 9. Failure Classes

## ENG-ERROR-10 — Hard Failures

Outcome = REJECTED

Includes:

- structural violations  
- hard invariant violations  
- domain schema violations  
- freeze-boundary violations  

---

## ENG-ERROR-11 — Blocking Failures

Outcome = BLOCKED

Includes:

- session-level blocking  
- candidate-level blocking  
- runtime blocking  

---

# 10. Candidate Model Integration

## ENG-ERROR-12 — Candidate Status Is Derived

Candidate status is not stored and must not be represented as an explicit state model.

Candidate status is derived from:

- presence or absence of violations  
- error_code classification  
- block_type  
- block_scope  

Eligibility and blocking semantics are defined by:

- ENG-DECISION  
- ENG-USABILITY  
- ENG-STRUCTURE  

---

# 11. Determinism Guarantees

## ENG-ERROR-13 — Evaluation Input Closure

Deterministic evaluation requires identical:

- domain graph  
- command input  
- runtime mode  
- spec_set_hash  
- schema_version  

---

## ENG-ERROR-14 — Deterministic Reporting

Given identical inputs:

- identical errors  
- identical ordering  
- identical outcome  
- identical primary_error_code  

The following fields may differ:

- evaluation_id  
- occurred_at  

---

## ENG-ERROR-15 — Canonical Equivalence

Two EvaluationReports are equivalent if all fields except:

- evaluation_id  
- occurred_at  

are identical.

---

## ENG-ERROR-16 — Canonical Representation

EvaluationReport must be canonically representable:

- fixed field order  
- deterministic ordering  
- explicit nulls  
- RFC 3339 timestamps  

---

# 12. Diagnostics Constraints

## ENG-ERROR-17 — Diagnostics Are Non-Semantic

diagnostics:

- must be deterministic for identical inputs  
- must not affect outcome  
- must not include canonical domain objects  
- must not encode hidden semantics  
- must be present or null deterministically  

---

# 13. No Implicit Repair

## ENG-ERROR-18 — Reporting Is Descriptive Only

The Engine must not:

- repair data  
- infer corrections  
- mutate state  
- resolve conflicts automatically  

---

# 14. Versioning

## ENG-ERROR-19 — Schema Versioning

Every report includes schema_version.

Changes to:

- structure  
- ordering  
- error codes  

require version increment.

---

# 15. Engine Invariants

- reporting is deterministic  
- reporting is complete  
- reporting never mutates state  
- reporting reflects authoritative specifications only  
- error vocabulary is closed  
- blocking is explicitly classified  
- candidate-level vs session-level blocking is always distinguishable  

---

# 16. Mental Model

ENG-ERROR defines reporting truth.

It answers:

- what failed  
- what is blocked  
- what is eligible  
- why  

It does not answer:

- how structure works  
- how usability works  
- how decisions are made  

It exposes those results deterministically.