# ENG-ERROR — Error & Evaluation Report Model  
Status: FROZEN (v3 – Orphan & Receipt Validation Added)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic evaluation results, failure reporting, and block classification  

---

# 1. Purpose

## ENG-ERROR-01 — Deterministic Evaluation Reporting

The Engine must produce a **structured, deterministic EvaluationReport** for every command.

This document defines:

- Error classification  
- Block classification  
- EvaluationReport schema  
- Deterministic failure codes  
- Hard vs soft rejection semantics  

The Engine must never:

- Fail silently  
- Emit ambiguous error messages  
- Depend on string messages for semantic meaning  

All semantic meaning must be encoded structurally.

---

# 2. Evaluation Model

## ENG-ERROR-02 — Every Command Returns an EvaluationReport

Every Engine API invocation must return exactly one:

EvaluationReport

This includes:

- Successful mutations  
- Rejections  
- Blocked operations  
- Deterministic no-op outcomes  

The Engine must not:

- Throw unstructured exceptions  
- Encode legitimacy state inside error strings

---

# 3. EvaluationReport Schema

An EvaluationReport must contain:

- evaluation_id (UUIDv7, engine-generated)  
- command_type (ENUM)  
- target_object_type (ENUM or null)  
- target_object_id (UUIDv7 or null)  
- outcome (ENUM)  
- error_code (ENUM or null)  
- block_type (ENUM or null)  
- related_objects (list of object references)  
- diagnostics (non-semantic, optional)  
- occurred_at (timestamp, deterministic format)  
- schema_version (string)

---

## 3.1 Outcome Enum

Exactly one of:

- SUCCESS  
- REJECTED  
- BLOCKED  
- NO_OP

Definitions:

- **SUCCESS** — Command executed and mutated engine state.  
- **REJECTED** — Command violated invariant, structural integrity, or legitimacy rule.  
- **BLOCKED** — Command rejected due to session block state or hygiene rule.  
- **NO_OP** — Command was valid but resulted in no state change.

---

## 3.2 BlockType Enum

Exactly one of:

- TEMPORARY  
- PERMANENT

Rules:

- block_type must be null unless outcome = BLOCKED  
- PERMANENT corresponds to BLOCK_PERMANENT hygiene invariant  
- TEMPORARY corresponds to BLOCK_TEMPORARY state

Fail if:

- BLOCKED outcome lacks block_type  
- block_type present when outcome != BLOCKED

---

# 4. Error Code Model

## ENG-ERROR-03 — Closed Set of Deterministic Error Codes

Error codes must be:

- Enumerated  
- Stable across engine versions  
- Deterministic  
- Machine-readable  

Error codes must not depend on:

- Localization  
- Message formatting  
- Runtime environment

---

## 4.1 Core Error Codes

### Structural Errors

- INVALID_UUID  
- DUPLICATE_ID  
- MISSING_REFERENCE  
- ORPHAN_REFERENCE_DETECTED *(new)*  
- INVALID_ENUM_VALUE  
- INVALID_STATE_COMBINATION  
- SUPERSESSION_CYCLE_DETECTED  
- RECEIPT_HASH_MISMATCH *(new)*

**Triggers:**

- `ORPHAN_REFERENCE_DETECTED`: Any command references an ID that does not exist in the engine’s loaded domain objects (sessions, resolutions, candidates, votes, authorities, scopes).  
- `RECEIPT_HASH_MISMATCH`: Session receipt content or canonical hash fails validation during restore or rehydration.

**Outcome:** REJECTED  
**related_objects:** The referencing object ID(s) and the missing or corrupted object ID(s).

---

### Session State Violations

- SESSION_TERMINAL_IMMUTABLE  
- SESSION_NOT_ACTIVE  
- SESSION_BLOCKED_TEMPORARY  
- SESSION_BLOCKED_PERMANENT

---

### Freeze Boundary Violations

- CANDIDATE_SET_FROZEN  
- PARTICIPANT_SET_FROZEN  
- CONSTRAINT_MUTATION_FORBIDDEN

---

### Participant Errors

- PARTICIPANT_NOT_FOUND  
- CANNOT_REMOVE_LAST_PARTICIPANT

---

### Context Violations

- AUTHORITY_CONTEXT_MISMATCH  
- SCOPE_CONTEXT_MISMATCH

---

### Acceptance & Legitimacy Violations

- ACCEPTANCE_CONDITIONS_NOT_MET  
- AREA_BLOCKED_BY_PERMANENT_SESSION  
- AUTHORITY_RULE_VIOLATION  
- CONSTRAINT_VIOLATION  
- SUPERSESSION_CONFLICT  
- RESOLUTION_ALREADY_SUPERSEDED

---

### Resolution & Lifecycle Errors

- INVALID_RESOLUTION_STATE_TRANSITION  
- RETIRED_STATE_VIOLATION  
- UNDER_REVIEW_STATE_VIOLATION  
- SNAPSHOT_INCOMPLETE

---

# 5. Hard vs Soft Failure

## ENG-ERROR-04 — Deterministic Failure Classes

**Hard Failures (REJECTED):**

- Structural invalidity (INVALID_UUID, DUPLICATE_ID, MISSING_REFERENCE, ORPHAN_REFERENCE_DETECTED, RECEIPT_HASH_MISMATCH)  
- Invariant violation  
- Authority violation  
- Constraint violation  
- Terminal immutability violation  
- Participant freeze violation  
- Attempt to remove last participant

**Soft Blocks (BLOCKED):**

- BLOCK_TEMPORARY session state  
- BLOCK_PERMANENT hygiene enforcement

Rules:

- Hard failures do not alter session state  
- Soft blocks do not alter session state  
- No automatic transitions permitted

---

# 6. Governance Hygiene Reporting

## ENG-ERROR-05 — Area Blocking Must Be Explicit

If acceptance fails due to BLOCK_PERMANENT hygiene:

- outcome = BLOCKED  
- block_type = PERMANENT  
- error_code = AREA_BLOCKED_BY_PERMANENT_SESSION  

The EvaluationReport must include:

- session_ids causing the block (in related_objects)

Engine must not:

- Auto-close blocking sessions  
- Suppress identification of blocking sessions

---

# 7. Terminal Session Rejection

## ENG-ERROR-06 — Closed & Accepted Sessions Are Immutable

If a command attempts to mutate a session where:

state ∈ {ACCEPTED, CLOSED}

Then:

- outcome = REJECTED  
- error_code = SESSION_TERMINAL_IMMUTABLE

No mutation permitted.

---

# 8. Determinism Guarantees

## ENG-ERROR-07 — Identical Inputs Produce Identical Reports

Given identical:

- Command input  
- Domain object graph  
- Session states  
- Authority rules  
- Scope rules

The Engine must produce identical:

- outcome  
- error_code  
- block_type  
- related_objects

Diagnostics may include additional context but must not alter semantics.

Fail if:

- Error ordering differs between implementations  
- Multiple valid error codes exist for same violation

---

# 9. Related Objects Field

## ENG-ERROR-08 — Explicit Context References

related_objects must include:

- All object IDs relevant to the failure  
- All sessions causing block  
- All constraints violated (if applicable)  
- Relevant participant_id when participant error occurs  
- Referenced or corrupted object IDs for orphan/receipt errors

This field:

- Must not imply legitimacy  
- Must not imply mutation  
- Exists solely for deterministic reporting

Fail if:

- Failure lacks relevant object context  
- Unrelated objects included

---

# 10. No Implicit Repair

## ENG-ERROR-09 — Engine Never Repairs

On any failure:

- Engine must not modify state  
- Engine must not downgrade block  
- Engine must not auto-resolve conflict  
- Engine must not auto-close session

Failures are descriptive only.

---

# 11. Audit Interaction

## ENG-ERROR-10 — Failures May Be Audited

Engine may emit audit events for:

- Rejected commands  
- Blocked attempts  

Audit emission must:

- Not alter legitimacy  
- Not alter session state  
- Be append-only  

EvaluationReport remains authoritative result of the command.

---

# 12. Versioning

Every EvaluationReport must include:

- schema_version

Error codes may only change under:

- Explicit engine version increment  
- Invariant review

---

# Summary Guarantees

- Every command yields structured output  
- No silent failure exists  
- Hard vs soft block is explicit  
- Freeze boundaries are enforced mechanically  
- Participant integrity is enforced (no empty sessions)  
- BLOCK_PERMANENT hygiene is transparent  
- Terminal sessions are immutable  
- Orphan and receipt validation are deterministic  
- Determinism preserved across implementations

---

# Mental Model

- Commands never “just fail.”  
- Every failure is classified.  
- Every block is explicit.  
- Participant integrity is enforced.  
- Orphan and receipt issues are deterministic hard failures.  
- Nothing mutates on rejection.  
- The engine explains mechanically, never narratively.

