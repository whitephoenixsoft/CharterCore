# ENG-ERROR — Engine Error & EvaluationReport Model
Status: FROZEN (v3 – Orphan & Receipt Validation Added)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic failure reporting, block classification, and evaluation output

---

# 1. Purpose

## ENG-ERROR-01 — Deterministic Evaluation Reporting

The Engine must produce a **structured, deterministic EvaluationReport** for every API command or operation.

This specification defines:

- Error classification
- Block classification
- EvaluationReport schema
- Deterministic error codes
- Hard vs soft failure semantics

**Requirements:**

- Failures must never be silent
- No semantic meaning may reside in free-form strings
- All failure context must be machine-readable

---

# 2. Evaluation Model

## ENG-ERROR-02 — Command Outcome Determinism

Every Engine API invocation returns **exactly one EvaluationReport**, including:

- Successful mutations
- Rejections due to invariant or governance violations
- Blocked operations (temporary or permanent)
- Deterministic no-op outcomes

The Engine must not throw unstructured exceptions or encode legitimacy within messages

---

# 3. EvaluationReport Schema

An EvaluationReport includes:

- `evaluation_id` — UUIDv7, engine-generated
- `command_type` — ENUM
- `target_object_type` — ENUM or null
- `target_object_id` — UUIDv7 or null
- `outcome` — ENUM (SUCCESS | REJECTED | BLOCKED | NO_OP)
- `error_code` — ENUM or null
- `block_type` — ENUM (TEMPORARY | PERMANENT) or null
- `related_objects` — list of object references
- `diagnostics` — optional, non-semantic context
- `occurred_at` — deterministic timestamp
- `schema_version` — string

---

## 3.1 Outcome Enum

- **SUCCESS** — Command executed and mutated engine state
- **REJECTED** — Command violated an invariant, structural rule, or governance precondition
- **BLOCKED** — Command rejected due to session block state or hygiene enforcement
- **NO_OP** — Command executed but caused no state change

---

## 3.2 BlockType Enum

- TEMPORARY → corresponds to `BLOCK_TEMPORARY` session state
- PERMANENT → corresponds to `BLOCK_PERMANENT` session or hygiene enforcement

**Rules:**

- `block_type` must be null unless `outcome = BLOCKED`
- Fail if `block_type` is present when `outcome != BLOCKED`
- Fail if `BLOCKED` outcome lacks a `block_type`

---

# 4. Error Codes

## ENG-ERROR-03 — Deterministic, Enumerated Codes

Error codes are:

- Explicitly enumerated
- Stable across engine versions
- Machine-readable and deterministic

Error codes must **never** depend on localization, formatting, or runtime behavior

---

### 4.1 Structural Errors

- `INVALID_UUID`
- `DUPLICATE_ID`
- `MISSING_REFERENCE`
- `ORPHAN_REFERENCE_DETECTED` *(new)*
- `INVALID_ENUM_VALUE`
- `INVALID_STATE_COMBINATION`
- `SUPERSESSION_CYCLE_DETECTED`
- `RECEIPT_HASH_MISMATCH` *(new)*

**Outcome:** REJECTED  
**Related objects:** referencing object + missing/corrupted object

---

### 4.2 Session State Violations

- `SESSION_TERMINAL_IMMUTABLE`
- `SESSION_NOT_ACTIVE`
- `SESSION_BLOCKED_TEMPORARY`
- `SESSION_BLOCKED_PERMANENT`

---

### 4.3 Freeze Boundary Violations

- `CANDIDATE_SET_FROZEN`
- `PARTICIPANT_SET_FROZEN`
- `CONSTRAINT_MUTATION_FORBIDDEN`

---

### 4.4 Participant Errors

- `PARTICIPANT_NOT_FOUND`
- `CANNOT_REMOVE_LAST_PARTICIPANT`

---

### 4.5 Context & Governance Violations

- `AUTHORITY_CONTEXT_MISMATCH`
- `SCOPE_CONTEXT_MISMATCH`

---

### 4.6 Acceptance & Legitimacy Violations

- `ACCEPTANCE_CONDITIONS_NOT_MET`
- `AREA_BLOCKED_BY_PERMANENT_SESSION`
- `AUTHORITY_RULE_VIOLATION`
- `CONSTRAINT_VIOLATION`
- `SUPERSESSION_CONFLICT`
- `RESOLUTION_ALREADY_SUPERSEDED`

---

### 4.7 Resolution & Lifecycle Errors

- `INVALID_RESOLUTION_STATE_TRANSITION`
- `RETIRED_STATE_VIOLATION`
- `UNDER_REVIEW_STATE_VIOLATION`
- `SNAPSHOT_INCOMPLETE`

---

# 5. Hard vs Soft Failure

## ENG-ERROR-04 — Deterministic Failure Classes

**Hard Failures (REJECTED):**

- Structural invalidity (UUID, duplicate IDs, missing references, orphan references, receipt hash mismatch)
- Invariant violations
- Authority or Scope rule violations
- Constraint violation
- Terminal immutability violation
- Freeze boundary violation
- Participant integrity violation

**Soft Blocks (BLOCKED):**

- BLOCK_TEMPORARY session state
- BLOCK_PERMANENT hygiene enforcement

**Rules:**

- Neither hard nor soft failures alter session or domain state
- No automatic transitions or implicit fixes are permitted

---

# 6. Governance Hygiene Reporting

## ENG-ERROR-05 — Explicit Area Blocking

When acceptance is blocked by `BLOCK_PERMANENT`:

- `outcome = BLOCKED`
- `block_type = PERMANENT`
- `error_code = AREA_BLOCKED_BY_PERMANENT_SESSION`
- `related_objects` includes session IDs causing the block

The Engine **must not** auto-close or suppress blocking sessions

---

# 7. Terminal Session Rejection

## ENG-ERROR-06 — Immutable Terminal Sessions

Mutating ACCEPTED or CLOSED sessions:

- `outcome = REJECTED`
- `error_code = SESSION_TERMINAL_IMMUTABLE`
- No mutation is allowed

---

# 8. Determinism Guarantees

## ENG-ERROR-07 — Identical Input → Identical Report

Given identical:

- Command input
- Domain graph
- Session states
- Authority and Scope rules

The Engine must produce identical:

- `outcome`
- `error_code`
- `block_type`
- `related_objects`

Fail if multiple valid error codes exist for a single violation or ordering differs between implementations

---

# 9. Related Objects Field

## ENG-ERROR-08 — Contextual References

`related_objects` must include:

- All object IDs relevant to the failure
- Blocking session IDs
- Violated constraints
- Relevant participant IDs
- Missing or corrupted object IDs

**Rules:**

- Must not imply legitimacy or state mutation
- Must only provide deterministic reporting context

---

# 10. No Implicit Repair

## ENG-ERROR-09 — Engine Never Modifies State

- Failures are descriptive only
- Engine must not downgrade blocks, auto-resolve conflicts, or auto-close sessions

---

# 11. Audit Interaction

## ENG-ERROR-10 — Failures Are Auditable

- Audit may record rejected or blocked commands
- Audit is append-only
- Audit events do not affect legitimacy or session state
- EvaluationReport remains authoritative

---

# 12. Versioning

- Every EvaluationReport includes `schema_version`
- Error codes may only change under explicit engine version increment

---

# Summary Guarantees

- Every command returns structured output
- Hard vs soft failure is explicit
- Freeze boundaries and participant integrity enforced
- BLOCK_PERMANENT hygiene is transparent
- Terminal sessions are immutable
- Orphan and receipt validation deterministic
- Determinism preserved across implementations

---

# Mental Model

- Commands **never fail silently**
- Every failure is classified and deterministic
- Blocks are explicit and descriptive
- Nothing mutates on rejection
- Orphan and receipt issues are deterministic **hard failures**
- The engine communicates mechanically, never narratively