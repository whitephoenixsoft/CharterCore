# ENG-ERROR — Engine Error & EvaluationReport Model  
Status: FROZEN (v8 – Canonical EvaluationReport Contract Added)  
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
- Participant epoch error semantics  
- EvaluationReport schema  
- Deterministic error codes  
- Hard vs soft failure semantics  
- Degraded read-only mode  
- Canonical representation guarantees  

Requirements:

- No silent failures  
- No semantic meaning in free-form strings  
- All violations machine-readable  
- Identical input → identical report (semantic and canonical form)  
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

Restore-time structural failures are not command responses, but must map to a deterministic structural error code defined here.

---

# 3. EvaluationReport Schema  

An EvaluationReport includes the following fields in fixed order:

1. evaluation_id — UUIDv7 (engine-generated)  
2. command_type — ENUM  
3. target_object_type — ENUM or null  
4. target_object_id — UUIDv7 or null  
5. outcome — ENUM (SUCCESS | REJECTED | BLOCKED | NO_OP)  
6. error_code — ENUM or null  
7. block_type — ENUM (TEMPORARY | PERMANENT) or null  
8. related_objects — ordered list of object references  
9. diagnostics — optional structured, non-semantic context  
10. occurred_at — deterministic timestamp  
11. schema_version — string  

No reordering permitted.

Future schema fields may only be appended to the end and require a schema_version increment.

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

- block_type must be null unless outcome = BLOCKED  
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

## 4.1 Structural Errors (Restore-Halting)

Indicate corruption or integrity violation.

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

### Receipt Integrity Errors (Structural)

- RECEIPT_MISSING  
- RECEIPT_HASH_MISMATCH  
- RECEIPT_ORPHAN_DETECTED  
- SNAPSHOT_PARTICIPANT_MISMATCH  

All receipt violations are structural integrity failures.

Restore behavior:

- Engine must halt  
- No evaluation permitted  
- No mutation permitted  

Outcome (command-time detection): REJECTED  

---

## 4.2 Degraded Mode Error  

- DEGRADED_MODE_ACTIVE  

In degraded mode:

- Only read-only operations and DAG export permitted  
- All mutating commands rejected deterministically  

---

## 4.3 Session State Violations  

- SESSION_TERMINAL_IMMUTABLE  
- SESSION_NOT_ACTIVE  
- SESSION_BLOCKED_TEMPORARY  
- SESSION_BLOCKED_PERMANENT  

---

## 4.4 Freeze Boundary Violations  

- CANDIDATE_SET_FROZEN  
- PARTICIPANT_SET_FROZEN  
- CONSTRAINT_MUTATION_FORBIDDEN  

---

## 4.5 Participant Errors  

- PARTICIPANT_NOT_FOUND  
- CANNOT_REMOVE_LAST_PARTICIPANT  
- DUPLICATE_PARTICIPANT_DISPLAY_NAME  
- PARTICIPANT_ALREADY_REMOVED  
- INVALID_PARTICIPANT_EPOCH  

Participant errors imply structural corruption only if detected during restore.

---

## 4.6 Governance & Context Violations  

- AUTHORITY_CONTEXT_MISMATCH  
- SCOPE_CONTEXT_MISMATCH  

---

## 4.7 Acceptance & Legitimacy Violations  

- ACCEPTANCE_CONDITIONS_NOT_MET  
- AREA_BLOCKED_BY_PERMANENT_SESSION  
- AUTHORITY_RULE_VIOLATION  
- CONSTRAINT_VIOLATION  
- SUPERSESSION_CONFLICT  
- RESOLUTION_ALREADY_SUPERSEDED  

---

## 4.8 Resolution & Lifecycle Errors  

- INVALID_RESOLUTION_STATE_TRANSITION  
- RETIRED_STATE_VIOLATION  
- UNDER_REVIEW_STATE_VIOLATION  
- SNAPSHOT_INCOMPLETE  

---

# 5. Hard vs Soft Failure  

## ENG-ERROR-04 — Deterministic Failure Classes  

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

# 6. Receipt Integrity Doctrine  

Receipts:

- Are persistent, immutable integrity artifacts  
- Must match canonical session snapshot  
- Snapshot participant IDs must match final epoch set  
- Must never be regenerated  

Receipt corruption requires halt on restore.

---

# 7. Determinism Guarantees  

## ENG-ERROR-05 — Semantic Determinism  

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

---

## ENG-ERROR-06 — Canonical Representation Guarantee  

EvaluationReport must be canonicalizable.

Identical input must produce a byte-identical canonical representation when serialized.

### Field Ordering

Fields must appear in the fixed order defined in Section 3.

### Collection Ordering

- related_objects → lexicographically sorted by object_id  
- diagnostics → lexicographically sorted by key  
- Any future list field → lexicographically sorted unless otherwise defined  

No insertion-order dependence permitted.

### Null Handling

- Optional fields must appear explicitly as null when not applicable  
- Fields must not be omitted conditionally  

### Serialization Stability

Canonical form must not depend on:

- Runtime environment  
- Map iteration order  
- Platform serializer behavior  
- Whitespace formatting  

Implementations may expose non-canonical representations, but must be able to emit canonical form deterministically.

---

# 8. Related Objects  

Must include relevant object identifiers in deterministic order.

Must not imply mutation or legitimacy semantics.

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
- Structural corruption halts deterministically  
- Participant epochs strictly enforced  
- Receipts are immutable integrity artifacts  
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