# ENG-ERROR — Engine Error & EvaluationReport Model  
Status: FROZEN (v5 – Receipt Integrity Formalized)  
Applies to: Engine Core (V1/V2+)  
Scope: Deterministic failure reporting, block classification, restore failure semantics, and evaluation output  

---

# 1. Purpose

## ENG-ERROR-01 — Deterministic Evaluation Reporting

The Engine must produce a structured, deterministic EvaluationReport for every API command.

This specification defines:

- Error classification  
- Block classification  
- Receipt integrity enforcement  
- EvaluationReport schema  
- Deterministic error codes  
- Hard vs soft failure semantics  

Requirements:

- No silent failures  
- No semantic meaning in free-form strings  
- All violations machine-readable  
- Identical input → identical report  
- Restore-time failures must classify deterministically  

---

# 2. Evaluation Model

## ENG-ERROR-02 — Command Outcome Determinism

Every Engine API invocation returns exactly one EvaluationReport:

- SUCCESS  
- REJECTED  
- BLOCKED  
- NO_OP  

The Engine must not:

- Throw unstructured exceptions  
- Encode legitimacy in text  
- Emit multiple competing classifications  

Restore-time structural failures are not command responses, but must map to a deterministic structural error class defined here.

---

# 3. EvaluationReport Schema

An EvaluationReport includes:

- `evaluation_id` — UUIDv7 (engine-generated)  
- `command_type` — ENUM  
- `target_object_type` — ENUM or null  
- `target_object_id` — UUIDv7 or null  
- `outcome` — ENUM (SUCCESS | REJECTED | BLOCKED | NO_OP)  
- `error_code` — ENUM or null  
- `block_type` — ENUM (TEMPORARY | PERMANENT) or null  
- `related_objects` — ordered list of object references  
- `diagnostics` — optional, non-semantic context  
- `occurred_at` — deterministic timestamp  
- `schema_version` — string  

---

## 3.1 Outcome Enum

- SUCCESS — Command mutated state  
- REJECTED — Invariant or structural violation  
- BLOCKED — Session or Area blocking condition  
- NO_OP — Valid command with no mutation  

---

## 3.2 BlockType Enum

- TEMPORARY  
- PERMANENT  

Rules:

- `block_type` must be null unless outcome = BLOCKED  
- BLOCKED must include block_type  
- REJECTED must not include block_type  

---

# 4. Error Codes

## ENG-ERROR-03 — Deterministic, Enumerated Codes

Error codes are:

- Explicit  
- Stable across versions  
- Deterministic  
- Mutually exclusive  

---

# 4.1 Structural Errors (Restore-Halting)

These indicate structural corruption or integrity violation.

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

### Receipt Integrity Errors (Structural)

Receipts are persistent integrity artifacts.

For every CLOSED or ACCEPTED session:

- Exactly one receipt must exist  
- Receipt must match deterministic canonical snapshot  
- Receipt must be immutable  
- Receipt must not be regenerated  

Violations:

- RECEIPT_MISSING  
- RECEIPT_HASH_MISMATCH  
- RECEIPT_ORPHAN_DETECTED  

Definitions:

- RECEIPT_MISSING → CLOSED or ACCEPTED session lacks a receipt  
- RECEIPT_HASH_MISMATCH → Persisted receipt does not match canonical snapshot  
- RECEIPT_ORPHAN_DETECTED → Receipt exists without matching CLOSED or ACCEPTED session  

All receipt violations are structural integrity failures.

Restore behavior:

- Engine must halt  
- No session evaluation permitted  
- No mutation permitted  

Outcome (command-time detection): REJECTED  

---

# 4.2 Session State Violations

- SESSION_TERMINAL_IMMUTABLE  
- SESSION_NOT_ACTIVE  
- SESSION_BLOCKED_TEMPORARY  
- SESSION_BLOCKED_PERMANENT  

---

# 4.3 Freeze Boundary Violations

- CANDIDATE_SET_FROZEN  
- PARTICIPANT_SET_FROZEN  
- CONSTRAINT_MUTATION_FORBIDDEN  

---

# 4.4 Participant Errors

- PARTICIPANT_NOT_FOUND  
- CANNOT_REMOVE_LAST_PARTICIPANT  

---

# 4.5 Governance & Context Violations

- AUTHORITY_CONTEXT_MISMATCH  
- SCOPE_CONTEXT_MISMATCH  

---

# 4.6 Acceptance & Legitimacy Violations

- ACCEPTANCE_CONDITIONS_NOT_MET  
- AREA_BLOCKED_BY_PERMANENT_SESSION  
- AUTHORITY_RULE_VIOLATION  
- CONSTRAINT_VIOLATION  
- SUPERSESSION_CONFLICT  
- RESOLUTION_ALREADY_SUPERSEDED  

These do not represent structural corruption.

---

# 4.7 Resolution & Lifecycle Errors

- INVALID_RESOLUTION_STATE_TRANSITION  
- RETIRED_STATE_VIOLATION  
- UNDER_REVIEW_STATE_VIOLATION  
- SNAPSHOT_INCOMPLETE  

---

# 5. Hard vs Soft Failure

## ENG-ERROR-04 — Deterministic Failure Classes

### Hard Failures (REJECTED)

Include:

- Structural graph violations  
- Supersession cycle  
- Multi-area detection  
- Cross-area supersession  
- Governance slot violation  
- Receipt violations  
- Invariant violations  
- Constraint violations  
- Terminal immutability  
- Freeze boundary violations  
- Participant integrity violations  

Hard failures:

- Never mutate state  
- Never trigger automatic repair  
- Deterministic classification required  

---

### Soft Blocks (BLOCKED)

Triggered by:

- BLOCK_TEMPORARY session state  
- BLOCK_PERMANENT hygiene enforcement  
- Area-level permanent block  

Soft blocks:

- Do not mutate domain objects  
- Are reversible only by explicit operator action  

---

# 6. Receipt Integrity Doctrine

## ENG-ERROR-05 — Receipts Are Persistent Integrity Artifacts

Receipts:

- Are not caches  
- Are not recomputable substitutes  
- Must not be regenerated  
- Must persist exactly as originally written  

Reproducing a receipt implies tampering.

Receipt absence implies corruption.

Receipt mismatch implies tampering.

Receipt without session implies corruption.

The Engine must halt on restore if receipt integrity cannot be proven.

No implicit regeneration allowed.

---

# 7. Determinism Guarantees

## ENG-ERROR-06 — Identical Input → Identical Report

Given identical:

- Active Area graph  
- Session states  
- Authority and Scope  
- Persisted receipts  
- Command input  

The Engine must produce identical:

- outcome  
- error_code  
- block_type  
- related_objects  

Ordering of related_objects must be deterministic.

---

# 8. Related Objects

## ENG-ERROR-07 — Contextual References

Must include:

- Corrupted object IDs  
- Blocking session IDs  
- Missing reference IDs  
- Governance slot objects  
- Receipt IDs (if violated)  

Must not imply mutation or legitimacy semantics.

---

# 9. No Implicit Repair

## ENG-ERROR-08 — Engine Never Modifies State on Failure

The Engine must not:

- Auto-generate missing receipts  
- Auto-correct receipt mismatches  
- Auto-repair slot multiplicity  
- Auto-resolve conflicts  
- Auto-close sessions  

Failures are descriptive only.

---

# 10. Audit Interaction

## ENG-ERROR-09 — Failures Are Auditable

- Audit may record rejected or blocked commands  
- Audit does not influence legitimacy  
- EvaluationReport remains authoritative  

---

# 11. Versioning

- Every EvaluationReport includes schema_version  
- Error codes may only change with explicit engine version increment  
- Receipt doctrine changes require version increment  

---

# Summary Guarantees

- Every command produces structured output  
- Structural corruption halts deterministically  
- Receipts are mandatory integrity artifacts  
- Receipt regeneration is prohibited  
- Governance slot violations are first-class structural failures  
- Cross-area supersession is prohibited  
- Determinism preserved across implementations  

---

# Mental Model

The engine compiles legitimacy.  
Receipts attest to what was compiled.  

If receipts are missing or altered,  
the engine cannot prove history —  
and must halt.