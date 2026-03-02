# ENG-ERROR — Engine Error & EvaluationReport Model  
Status: FROZEN (v10 – Multi-Error Canonical Contract with Restored Doctrine)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic failure reporting, block classification, restore failure semantics, receipt enforcement, participant epoch enforcement, and canonical evaluation output  

---

# 1. Purpose  

## ENG-ERROR-01 — Deterministic Evaluation Reporting  

The Engine must produce a structured, deterministic EvaluationReport for every API command.

This specification defines:

- Error classification  
- Block classification  
- Receipt integrity enforcement  
- Participant epoch enforcement  
- Multi-error accumulation semantics  
- EvaluationReport schema  
- Deterministic error codes  
- Hard vs soft failure semantics  
- Degraded read-only mode  
- Canonical representation guarantees  

Requirements:

- No silent failures  
- No semantic meaning in free-form strings  
- All violations machine-readable  
- No short-circuit evaluation  
- Identical input → identical report (semantic and canonical form)  
- Restore-time failures must classify deterministically  

---

# 2. Evaluation Model  

## ENG-ERROR-02 — Full Deterministic Evaluation Pass  

Every Engine API invocation must:

- Execute a full validation pass  
- Evaluate violations in defined phase order  
- Accumulate all detected violations  
- Derive outcome only after evaluation completes  
- Never short-circuit on first error  

Each invocation returns exactly one EvaluationReport with one outcome:

- SUCCESS  
- REJECTED  
- BLOCKED  
- NO_OP  

The Engine must not:

- Throw unstructured exceptions  
- Encode legitimacy in text  
- Emit multiple competing classifications  

Restore-time structural failures are not command responses but must map to deterministic structural error entries defined here.

---

# 3. Deterministic Evaluation Phases  

## ENG-ERROR-03 — Fixed Phase Ordering  

Violations must be evaluated and accumulated in the following order:

1. Structural graph validation  
2. Receipt integrity validation  
3. Governance slot validation  
4. Session state validation  
5. Freeze boundary validation  
6. Participant validation and epoch enforcement  
7. Resolution and lifecycle validation  
8. Acceptance and constraint validation  
9. Block condition evaluation  

No reordering permitted.

Within each phase:

- Errors must be sorted lexicographically by error_code  
- Then lexicographically by related object identifiers  

---

# 4. EvaluationReport Schema  

Fields must appear in the following fixed order:

1. evaluation_id — UUIDv7 (engine-generated)  
2. command_type — ENUM  
3. target_object_type — ENUM or null  
4. target_object_id — UUIDv7 or null  
5. outcome — ENUM (SUCCESS | REJECTED | BLOCKED | NO_OP)  
6. errors — ordered list of ErrorEntry  
7. primary_error_code — ENUM or null (derived)  
8. diagnostics — optional structured, non-semantic context  
9. occurred_at — RFC 3339 UTC timestamp (YYYY-MM-DDTHH:MM:SS.mmmZ)  
10. schema_version — string  

No field reordering permitted.  
Future fields may only be appended and require schema_version increment.

---

## 4.1 ErrorEntry Structure  

Each ErrorEntry contains:

- error_code — ENUM  
- related_objects — ordered list of object identifiers  
- block_type — ENUM (TEMPORARY | PERMANENT) or null  

Rules:

- related_objects must be lexicographically sorted  
- block_type must be non-null only for blocking conditions  
- Each error_code may appear at most once per EvaluationReport  

---

## 4.2 Outcome Derivation  

Outcome must be derived after full evaluation using the following precedence:

1. If any structural error present → REJECTED  
2. Else if any hard invariant violation present → REJECTED  
3. Else if any blocking condition present → BLOCKED  
4. Else if mutation occurred → SUCCESS  
5. Else → NO_OP  

primary_error_code must equal the first error_code in the ordered errors list, or null if errors is empty.

---

# 5. Error Codes  

## ENG-ERROR-04 — Deterministic, Enumerated Codes  

Error codes are:

- Explicit  
- Stable across versions  
- Deterministic  
- Mutually exclusive  

---

## 5.1 Structural Errors (Restore-Halting)

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

### Receipt Structural Errors

- RECEIPT_MISSING  
- RECEIPT_HASH_MISMATCH  
- RECEIPT_ORPHAN_DETECTED  
- SNAPSHOT_PARTICIPANT_MISMATCH  

Restore behavior:

- Engine must halt  
- No evaluation permitted  
- No mutation permitted  

Command-time detection outcome: REJECTED  

---

## 5.2 Degraded Mode Error  

- DEGRADED_MODE_ACTIVE  

In degraded mode:

- Only read-only operations and DAG export permitted  
- All mutating commands rejected deterministically  

---

## 5.3 Session State Violations  

- SESSION_TERMINAL_IMMUTABLE  
- SESSION_NOT_ACTIVE  
- SESSION_BLOCKED_TEMPORARY  
- SESSION_BLOCKED_PERMANENT  

---

## 5.4 Freeze Boundary Violations  

- CANDIDATE_SET_FROZEN  
- PARTICIPANT_SET_FROZEN  
- CONSTRAINT_MUTATION_FORBIDDEN  

---

## 5.5 Participant Errors  

- PARTICIPANT_NOT_FOUND  
- CANNOT_REMOVE_LAST_PARTICIPANT  
- DUPLICATE_PARTICIPANT_DISPLAY_NAME  
- PARTICIPANT_ALREADY_REMOVED  
- INVALID_PARTICIPANT_EPOCH  

Participant errors imply structural corruption only if detected during restore.

---

## 5.6 Governance & Context Violations  

- AUTHORITY_CONTEXT_MISMATCH  
- SCOPE_CONTEXT_MISMATCH  

---

## 5.7 Acceptance & Legitimacy Violations  

- ACCEPTANCE_CONDITIONS_NOT_MET  
- AREA_BLOCKED_BY_PERMANENT_SESSION  
- AUTHORITY_RULE_VIOLATION  
- CONSTRAINT_VIOLATION  
- SUPERSESSION_CONFLICT  
- RESOLUTION_ALREADY_SUPERSEDED  

---

## 5.8 Resolution & Lifecycle Errors  

- INVALID_RESOLUTION_STATE_TRANSITION  
- RETIRED_STATE_VIOLATION  
- UNDER_REVIEW_STATE_VIOLATION  
- SNAPSHOT_INCOMPLETE  

---

# 6. Hard vs Soft Failure  

## ENG-ERROR-05 — Deterministic Failure Classes  

### Hard Failures (REJECTED)

- Structural violations  
- Governance slot violations  
- Receipt violations  
- Snapshot participant mismatch  
- Participant ID reuse  
- Invariant violations  
- Freeze boundary violations  

Hard failures:

- Never mutate state  
- Never trigger automatic repair  

### Soft Blocks (BLOCKED)

- BLOCK_TEMPORARY session state  
- BLOCK_PERMANENT hygiene enforcement  
- Area-level permanent block  

Soft blocks:

- Do not mutate domain objects  
- Reversible only by explicit operator action  

---

# 7. Receipt Integrity Doctrine  

Receipts are constitutional integrity artifacts.

They:

- Are persistent and immutable  
- Are not caches  
- Are not recomputable substitutes  
- Must match canonical session snapshot  
- Must bind to final participant epoch set  
- Must never be regenerated implicitly  

If receipt integrity cannot be proven:

- The Engine must halt on restore  
- No evaluation permitted  
- No mutation permitted  

Receipt corruption implies inability to prove legitimacy history.

---

# 8. Determinism Guarantees  

## ENG-ERROR-06 — Semantic Determinism  

Given identical:

- Area graph  
- Session states  
- Authority and Scope  
- Persisted receipts  
- Command input  

The Engine must produce identical:

- errors list  
- error ordering  
- primary_error_code  
- outcome  
- related_objects  

---

## ENG-ERROR-07 — Canonical Determinism  

EvaluationReport must be canonicalizable.

Identical input must produce byte-identical canonical representation.

### Ordering Rules

- Fields must appear in fixed schema order  
- errors list ordered by:
  1. Evaluation phase  
  2. error_code (lexicographic)  
  3. related object identifiers (lexicographic)  
- related_objects sorted lexicographically  
- diagnostics sorted lexicographically by key  

### Null Handling

- Optional fields must appear explicitly as null  
- Fields must not be conditionally omitted  

### Timestamp Format

occurred_at must be:

- RFC 3339  
- UTC only  
- Fixed millisecond precision  
- Format: YYYY-MM-DDTHH:MM:SS.mmmZ  

Canonical form must not depend on:

- Runtime environment  
- Map iteration order  
- Serializer behavior  
- Whitespace formatting  

---

# 9. No Implicit Repair  

The Engine must not:

- Regenerate receipts  
- Repair slot violations  
- Reconcile participant epochs  
- Auto-resolve conflicts  
- Auto-close sessions  

Failures are descriptive only.

---

# 10. Versioning  

- Every EvaluationReport includes schema_version  
- Schema changes require explicit version increment  
- Error code changes require version increment  

---

# 11. Summary Guarantees  

- Every command produces structured output  
- All violations accumulated deterministically  
- Outcome derived from highest-severity violation  
- Structural corruption halts deterministically  
- Participant epochs strictly enforced  
- Receipts are immutable constitutional artifacts  
- EvaluationReport is semantically and canonically deterministic  
- Cross-implementation output stability guaranteed  
- Degraded read-only mode permits recovery export only  

---

# 12. Mental Model  

The engine compiles legitimacy.  
Receipts attest to what was compiled.  
Participant IDs represent participation epochs.  
EvaluationReports describe the compiler’s decision deterministically and canonically.  

If structure, receipts, or participant epochs are inconsistent,  
the engine cannot prove history —  
and must halt.