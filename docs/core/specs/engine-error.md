# ENG-ERROR — Engine Error & EvaluationReport Model  
Status: FROZEN (v4 – Single-Area & Governance Slot Invariants Formalized)  
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

Requirements:

- Failures must never be silent  
- No semantic meaning may reside in free-form strings  
- All failure context must be machine-readable  
- Identical input conditions must yield identical reports  

---

# 2. Evaluation Model

## ENG-ERROR-02 — Command Outcome Determinism

Every Engine API invocation returns **exactly one EvaluationReport**, including:

- Successful mutations  
- Rejections due to invariant or governance violations  
- Blocked operations (temporary or permanent)  
- Deterministic no-op outcomes  

The Engine must not:

- Throw unstructured exceptions  
- Encode legitimacy within messages  
- Return multiple competing failure classifications  

Structural restore failures during initialization are not command responses but must still resolve to a deterministic structural error classification defined in this document.

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
- **REJECTED** — Command violated a structural rule, invariant, or governance precondition  
- **BLOCKED** — Command rejected due to session block state or hygiene enforcement  
- **NO_OP** — Command executed but caused no state change  

---

## 3.2 BlockType Enum

- TEMPORARY → corresponds to `BLOCK_TEMPORARY` session state  
- PERMANENT → corresponds to `BLOCK_PERMANENT` session or hygiene enforcement  

Rules:

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
- Mutually exclusive per violation condition  

Error codes must never depend on localization, formatting, ordering, or runtime heuristics.

---

## 4.1 Structural Errors

These represent invariant violations or graph corruption.

- `INVALID_UUID`  
- `DUPLICATE_ID`  
- `MISSING_REFERENCE`  
- `ORPHAN_REFERENCE_DETECTED`  
- `INVALID_ENUM_VALUE`  
- `INVALID_STATE_COMBINATION`  
- `SUPERSESSION_CYCLE_DETECTED`  
- `RECEIPT_HASH_MISMATCH`  
- `MULTI_AREA_GRAPH_DETECTED`  
- `CROSS_AREA_SUPERSESSION_PROHIBITED`  
- `GOVERNANCE_SLOT_EMPTY`  
- `GOVERNANCE_SLOT_MULTIPLICITY`  

Definitions:

- **MULTI_AREA_GRAPH_DETECTED** — Structural objects from more than one Area exist in a single engine instance.  
- **CROSS_AREA_SUPERSESSION_PROHIBITED** — A supersession edge targets a Resolution from a different Area.  
- **GOVERNANCE_SLOT_EMPTY** — An exclusive governance slot (Authority or Scope) has zero structurally ACTIVE objects when required.  
- **GOVERNANCE_SLOT_MULTIPLICITY** — An exclusive governance slot contains more than one structurally ACTIVE object.  

Outcome: REJECTED (for commands)  
Restore: StructuralIntegrityFailure → Engine must halt  

---

## 4.2 Session State Violations

- `SESSION_TERMINAL_IMMUTABLE`  
- `SESSION_NOT_ACTIVE`  
- `SESSION_BLOCKED_TEMPORARY`  
- `SESSION_BLOCKED_PERMANENT`  

---

## 4.3 Freeze Boundary Violations

- `CANDIDATE_SET_FROZEN`  
- `PARTICIPANT_SET_FROZEN`  
- `CONSTRAINT_MUTATION_FORBIDDEN`  

---

## 4.4 Participant Errors

- `PARTICIPANT_NOT_FOUND`  
- `CANNOT_REMOVE_LAST_PARTICIPANT`  

---

## 4.5 Context & Governance Violations

- `AUTHORITY_CONTEXT_MISMATCH`  
- `SCOPE_CONTEXT_MISMATCH`  

---

## 4.6 Acceptance & Legitimacy Violations

- `ACCEPTANCE_CONDITIONS_NOT_MET`  
- `AREA_BLOCKED_BY_PERMANENT_SESSION`  
- `AUTHORITY_RULE_VIOLATION`  
- `CONSTRAINT_VIOLATION`  
- `SUPERSESSION_CONFLICT`  
- `RESOLUTION_ALREADY_SUPERSEDED`  

These represent evaluation-time failures that do not indicate structural corruption.

---

## 4.7 Resolution & Lifecycle Errors

- `INVALID_RESOLUTION_STATE_TRANSITION`  
- `RETIRED_STATE_VIOLATION`  
- `UNDER_REVIEW_STATE_VIOLATION`  
- `SNAPSHOT_INCOMPLETE`  

---

# 5. Hard vs Soft Failure

## ENG-ERROR-04 — Deterministic Failure Classes

### Hard Failures (REJECTED)

Include:

- Structural invalidity  
- Supersession cycle  
- Multi-area graph detection  
- Cross-area supersession attempt  
- Governance slot emptiness or multiplicity  
- Invariant violations  
- Constraint violation  
- Terminal immutability violation  
- Freeze boundary violation  
- Participant integrity violation  

Hard failures:

- Do not mutate domain state  
- Do not trigger automatic transitions  
- Must be deterministic  

---

### Soft Blocks (BLOCKED)

Triggered by:

- BLOCK_TEMPORARY session state  
- BLOCK_PERMANENT hygiene enforcement  
- Area-level permanent blocking  

Rules:

- Neither hard nor soft failures alter domain objects  
- No automatic repairs permitted  

---

# 6. Governance Hygiene Reporting

## ENG-ERROR-05 — Explicit Area Blocking

When acceptance is blocked by `BLOCK_PERMANENT`:

- `outcome = BLOCKED`  
- `block_type = PERMANENT`  
- `error_code = AREA_BLOCKED_BY_PERMANENT_SESSION`  
- `related_objects` includes blocking session IDs  

The Engine must not auto-close or suppress blocking sessions.

---

# 7. Terminal Session Rejection

## ENG-ERROR-06 — Immutable Terminal Sessions

Mutating ACCEPTED or CLOSED sessions:

- `outcome = REJECTED`  
- `error_code = SESSION_TERMINAL_IMMUTABLE`  

No mutation is allowed.

---

# 8. Determinism Guarantees

## ENG-ERROR-07 — Identical Input → Identical Report

Given identical:

- Command input  
- Active Area domain graph  
- Session states  
- Authority and Scope rules  

The Engine must produce identical:

- `outcome`  
- `error_code`  
- `block_type`  
- `related_objects`  

Fail if:

- Multiple valid error codes exist for a single violation  
- Ordering differs between implementations  
- Mixed-area graphs are classified inconsistently  

Determinism applies strictly within a single active Area.

---

# 9. Related Objects Field

## ENG-ERROR-08 — Contextual References

`related_objects` must include:

- All object IDs relevant to the failure  
- Blocking session IDs  
- Violated constraint IDs  
- Relevant participant IDs  
- Missing or corrupted object IDs  
- Governance slot objects (if slot invariant violated)  

Rules:

- Must not imply legitimacy  
- Must not imply mutation  
- Deterministic ordering required  

---

# 10. No Implicit Repair

## ENG-ERROR-09 — Engine Never Modifies State

The Engine must not:

- Auto-close sessions  
- Auto-resolve supersession conflicts  
- Auto-downgrade blocks  
- Auto-repair slot multiplicity  
- Auto-correct multi-area hosting  

Failures are descriptive only.

---

# 11. Audit Interaction

## ENG-ERROR-10 — Failures Are Auditable

- Audit may record rejected or blocked commands  
- Audit is append-only  
- Audit does not influence legitimacy  
- EvaluationReport remains authoritative  

---

# 12. Versioning

- Every EvaluationReport includes `schema_version`  
- Error codes may only change under explicit engine version increment  
- Additions require deterministic classification guarantee  

---

# Summary Guarantees

- Every command returns structured output  
- Hard vs soft failure is explicit  
- Single-Area runtime enforced  
- Governance slot invariants explicitly classified  
- Cross-area supersession explicitly prohibited  
- No silent structural ambiguity  
- Determinism preserved across implementations  

---

# Mental Model

- Commands never fail silently  
- Every violation has a unique mechanical identity  
- Blocks are explicit and typed  
- Nothing mutates on rejection  
- Multi-area corruption halts deterministically  
- Governance slot violations are first-class structural failures  
- The engine communicates mechanically, never narratively