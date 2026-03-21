# ENG-ERROR — Engine Error & EvaluationReport Model

Status: REFACTORED (v11 – Reference-Driven Reporting Contract)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic reporting, EvaluationReport structure, error classification, and canonical output

Authority: Foundational authority for Engine error representation and EvaluationReport semantics.

Subordinate references consumed from:

- ENG-INTEGRITY
- ENG-INITIALIZATION
- ENG-DECISION
- ENG-SESSION
- ENG-SUPERSESSION
- ENG-RECEIPT
- ENG-CANON
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
- stable error code vocabulary

ENG-ERROR does not define:

- structural validation rules
- halt conditions
- degraded mode eligibility
- session lifecycle rules
- acceptance rules
- supersession graph semantics
- receipt structure or receipt validity rules

Those are defined elsewhere, especially in:

- ENG-INTEGRITY
- ENG-INITIALIZATION
- ENG-DECISION
- ENG-SESSION
- ENG-SUPERSESSION
- ENG-RECEIPT

ENG-ERROR defines how those outcomes are reported, not how they are decided.

---

# 2. Core Reporting Principle

## ENG-ERROR-01 — Deterministic Structured Reporting

The Engine must produce a structured, deterministic EvaluationReport for every API command that returns command-level reporting.

Requirements:

- no silent failures
- no semantic meaning in free-form strings
- all violations machine-readable
- no first-error short-circuit reporting
- identical input must produce identical report semantics
- canonical output form must be deterministic

Restore-time and initialization failures are not ordinary session mutations, but their reported violations must still map into the deterministic error vocabulary defined here when surfaced through Engine reporting interfaces.

---

# 3. Evaluation Model

## ENG-ERROR-02 — Full Deterministic Evaluation Pass

For command reporting, the Engine must:

- execute a full validation pass appropriate to the invoked operation
- accumulate all detected violations
- derive outcome only after evaluation completes
- never short-circuit reporting on the first error

Each EvaluationReport has exactly one outcome:

- SUCCESS
- REJECTED
- BLOCKED
- NO_OP

The Engine must not:

- throw unstructured semantic exceptions as the reporting model
- emit multiple competing outcomes
- encode legitimacy meaning in narrative text

ENG-ERROR defines the reporting model only.  
Command behavior is defined in ENG-API and underlying authoritative specifications.

---

# 4. Evaluation Phase Ordering

## ENG-ERROR-03 — Fixed Ordering of Reported Violations

When multiple violations are reported in one EvaluationReport, they must be ordered deterministically by validation phase.

Default reporting phase order:

1. Structural graph validation
2. Receipt integrity validation
3. Governance slot validation
4. Session state validation
5. Freeze boundary validation
6. Participant validation and epoch enforcement
7. Resolution and lifecycle validation
8. Acceptance and constraint validation
9. Block condition evaluation

Within each phase:

- errors sorted lexicographically by error_code
- then lexicographically by related object identifiers

ENG-ERROR defines the ordering of reported violations.  
It does not require every command to execute every phase if a phase is not applicable to that command type.

---

# 5. EvaluationReport Schema

## ENG-ERROR-04 — Fixed Report Shape

EvaluationReport fields must appear in the following fixed order:

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

Field meanings:

- evaluation_id — Engine-generated UUIDv7
- command_type — command identifier
- target_object_type — ENUM or null
- target_object_id — UUIDv7 or null
- outcome — one of SUCCESS | REJECTED | BLOCKED | NO_OP
- errors — ordered list of ErrorEntry
- primary_error_code — derived first error_code or null
- diagnostics — optional structured non-semantic context
- occurred_at — RFC 3339 UTC timestamp with fixed millisecond precision
- schema_version — report schema version

No field reordering permitted.  
Future fields may only be appended with schema_version increment.

---

## ENG-ERROR-05 — ErrorEntry Structure

Each ErrorEntry contains:

- error_code
- related_objects
- block_type

Rules:

- related_objects must be lexicographically sorted
- block_type must be non-null only for blocking conditions
- each error_code may appear at most once per EvaluationReport

block_type values:

- TEMPORARY
- PERMANENT
- null

ENG-ERROR defines structure only.  
Whether a given condition is blocking is determined by the authoritative behavioral specification and then represented here.

---

# 6. Outcome Derivation

## ENG-ERROR-06 — Outcome Precedence

Outcome is derived after full evaluation using this precedence:

1. If any structural error present → REJECTED
2. Else if any hard invariant violation present → REJECTED
3. Else if any blocking condition present → BLOCKED
4. Else if mutation occurred → SUCCESS
5. Else → NO_OP

primary_error_code must equal the first error_code in the ordered errors list, or null if errors is empty.

ENG-ERROR defines outcome derivation once violations are known.  
It does not define which violations exist.

---

# 7. Error Code Vocabulary

## ENG-ERROR-07 — Stable Enumerated Error Codes

Error codes must be:

- explicit
- stable
- deterministic
- versioned through this specification

ENG-ERROR is the authority for the error-code vocabulary.

---

## 7.1 Structural Errors

Structural errors represent failures of graph or runtime-entry validity.

Defined structural error codes:

- INVALID_UUID
- DUPLICATE_ID
- MISSING_REFERENCE
- ORPHAN_REFERENCE_DETECTED
- INVALID_ENUM_VALUE
- INVALID_STATE_COMBINATION
- SUPERSESSION_CYCLE_DETECTED
- MULTI_AREA_GRAPH_DETECTED
- CROSS_AREA_SUPERSESSION_PROHIBITED
- GOVERNANCE_SLOT_EMPTY
- GOVERNANCE_SLOT_MULTIPLICITY
- PARTICIPANT_ID_REUSE_DETECTED

Receipt-related structural error codes:

- RECEIPT_MISSING
- RECEIPT_HASH_MISMATCH
- RECEIPT_ORPHAN_DETECTED
- SNAPSHOT_PARTICIPANT_MISMATCH

ENG-ERROR defines these codes and their reporting class.  
ENG-INTEGRITY and ENG-INITIALIZATION determine when they arise and whether runtime must halt.

---

## 7.2 Degraded Mode Error

Defined degraded mode error code:

- DEGRADED_MODE_ACTIVE

This code is used when a mutating operation is rejected because runtime mode forbids mutation.

Degraded mode policy is defined in ENG-INTEGRITY and ENG-INITIALIZATION.

---

## 7.3 Session State Violations

Defined session-state error codes:

- SESSION_TERMINAL_IMMUTABLE
- SESSION_NOT_ACTIVE
- SESSION_BLOCKED_TEMPORARY
- SESSION_BLOCKED_PERMANENT

These codes represent command-time session state reporting.  
Session lifecycle rules belong to ENG-SESSION.

---

## 7.4 Freeze Boundary Violations

Defined freeze-boundary error codes:

- CANDIDATE_SET_FROZEN
- PARTICIPANT_SET_FROZEN
- CONSTRAINT_MUTATION_FORBIDDEN

Freeze-boundary semantics belong to ENG-SESSION and ENG-DECISION.  
ENG-ERROR defines how they are reported.

---

## 7.5 Participant Errors

Defined participant-related error codes:

- PARTICIPANT_NOT_FOUND
- CANNOT_REMOVE_LAST_PARTICIPANT
- DUPLICATE_PARTICIPANT_DISPLAY_NAME
- PARTICIPANT_ALREADY_REMOVED
- INVALID_PARTICIPANT_EPOCH

These may represent command-time errors or structural restore violations depending on context.

---

## 7.6 Governance & Context Violations

Defined governance-context error codes:

- AUTHORITY_CONTEXT_MISMATCH
- SCOPE_CONTEXT_MISMATCH

These codes report governance-context failures surfaced by underlying validation rules.

---

## 7.7 Acceptance & Legitimacy Violations

Defined acceptance-related error codes:

- ACCEPTANCE_CONDITIONS_NOT_MET
- AREA_BLOCKED_BY_PERMANENT_SESSION
- AUTHORITY_RULE_VIOLATION
- CONSTRAINT_VIOLATION
- SUPERSESSION_CONFLICT
- RESOLUTION_ALREADY_SUPERSEDED

Acceptance rules are not defined here.  
They are represented here.

---

## 7.8 Resolution & Lifecycle Errors

Defined lifecycle-related error codes:

- INVALID_RESOLUTION_STATE_TRANSITION
- RETIRED_STATE_VIOLATION
- UNDER_REVIEW_STATE_VIOLATION
- SNAPSHOT_INCOMPLETE

Lifecycle semantics are defined elsewhere.  
ENG-ERROR provides stable reporting codes.

---

# 8. Failure Classes

## ENG-ERROR-08 — Hard Failures

Hard failures produce outcome = REJECTED.

Hard failures include:

- structural violations
- governance slot violations
- receipt violations
- participant epoch reuse
- invariant violations
- freeze-boundary violations
- other non-blocking failures that prevent operation validity

Hard failures:

- must never mutate state
- must never trigger automatic repair

---

## ENG-ERROR-09 — Soft Blocks

Soft blocks produce outcome = BLOCKED.

Soft blocks include command-valid but currently non-executable conditions such as:

- reversible blocking state
- permanent blocking state where command remains reportable as blocked
- Area-level blocking conditions

Soft blocks:

- do not mutate state
- remain descriptive only
- require external or explicit action according to underlying behavioral rules

ENG-ERROR defines reporting classification only.  
It does not define what causes a block.

---

# 9. Receipt Reporting Doctrine

## ENG-ERROR-10 — Receipt Failures Are Reported, Not Repaired

Receipt-related failures must be represented using deterministic error codes.

ENG-ERROR does not define receipt validity or halt policy.  
Those belong to:

- ENG-RECEIPT
- ENG-CANON
- ENG-SPECVERIFY
- ENG-INTEGRITY

ENG-ERROR defines only how receipt failures surface in reports.

The Engine must not use reporting as a repair mechanism.

---

# 10. Canonical Determinism

## ENG-ERROR-11 — Semantic Determinism

Given identical:

- runtime graph
- command input
- runtime mode
- authoritative validation outcomes

the Engine must produce identical:

- errors list
- error ordering
- primary_error_code
- outcome
- related_objects

---

## ENG-ERROR-12 — Canonical Report Determinism

EvaluationReport must be canonicalizable.

Canonical report rules:

- fields appear in fixed schema order
- errors ordered by reporting phase, then error_code, then related_objects
- related_objects sorted lexicographically
- diagnostics sorted lexicographically by key
- optional fields explicitly present as null when absent
- occurred_at formatted as RFC 3339 UTC with fixed millisecond precision

Canonical byte-level serialization rules are consumed from ENG-CANON where applicable to report serialization policy.

ENG-ERROR defines report ordering and schema, not general Engine canonicalization rules.

---

# 11. No Implicit Repair

## ENG-ERROR-13 — Reporting Is Descriptive Only

The Engine must not use errors to imply or perform automatic repair.

The Engine must not:

- regenerate receipts
- repair governance slots
- reconcile participant epochs
- auto-resolve conflicts
- auto-close sessions

Error reporting is descriptive.  
Repair, if any, must be explicit and external to this reporting contract.

---

# 12. Versioning

## ENG-ERROR-14 — Report Schema Versioning

Every EvaluationReport must include schema_version.

Changes to:

- EvaluationReport structure
- ErrorEntry structure
- outcome model
- error code vocabulary

require a version increment.

ENG-ERROR is the versioning authority for the report contract.

---

# 13. Engine Invariants

- every command-level reportable operation yields deterministic structured output
- outcome derived only after full applicable evaluation completes
- errors accumulated deterministically
- report ordering stable across implementations
- error-code vocabulary stable within schema version
- reporting never mutates state
- reporting never substitutes for validation logic
- dependent specifications must use ENG-ERROR rather than redefining reporting structure or error precedence

---

# 14. Mental Model

ENG-ERROR defines reporting truth.

It answers:

- how Engine failures and blocks are represented
- how multiple violations are ordered
- how outcomes are derived
- how reports remain deterministic across implementations

It does not answer:

- what structural rules exist
- when the Engine halts
- how sessions behave
- how legitimacy is decided
- how receipts are verified

Those belong elsewhere.

ENG-ERROR is the reporting layer.  
Other specifications must consume it rather than redefine report semantics.